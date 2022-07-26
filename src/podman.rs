//! Main entrypoint for interacting with the Podman API.

use crate::{
    api::{self, ApiResource},
    conn::{get_http_connector, transport, Headers, Payload, Transport},
    models,
    opts::*,
    ApiVersion, Error, Result, LATEST_API_VERSION,
};

#[cfg(feature = "tls")]
use crate::conn::get_https_connector;
#[cfg(unix)]
use crate::conn::get_unix_connector;

use crate::conn::hyper::{body::Bytes, Body, Client, Method, Response};

use containers_api::url;
use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
    TryStreamExt,
};
use log::trace;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Entrypoint interface for communicating with podman daemon
#[derive(Debug, Clone)]
pub struct Podman {
    version: ApiVersion,
    transport: Transport,
}

impl Podman {
    /// Creates a new Podman instance by automatically choosing appropriate connection type based
    /// on provided `uri`.
    ///
    /// Supported schemes are:
    ///  - `unix://` only works when build target is `unix`, otherwise returns an Error
    ///  - `tcp://`
    ///  - `http://`
    ///
    ///  To create a Podman instance utilizing TLS use explicit [Podman::tls](Podman::tls)
    ///  constructor (this requires `tls` feature enabled).
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Podman::new_versioned`](Podman::new_versioned).
    pub fn new<U>(uri: U) -> Result<Podman>
    where
        U: AsRef<str>,
    {
        Self::new_versioned(uri, LATEST_API_VERSION)
    }

    /// Same as [`Podman::new`](Podman::new) but the API version can be explicitly specified.
    pub fn new_versioned<U>(uri: U, version: impl Into<ApiVersion>) -> Result<Podman>
    where
        U: AsRef<str>,
    {
        let version = version.into();
        let uri = uri.as_ref();
        let mut it = uri.split("://");

        match it.next() {
            #[cfg(unix)]
            Some("unix") => {
                if let Some(path) = it.next() {
                    Ok(Podman::unix_versioned(path, version))
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            #[cfg(not(unix))]
            Some("unix") => Err(Error::UnsupportedScheme("unix".to_string())),
            Some("tcp") | Some("http") => {
                if let Some(host) = it.next() {
                    Podman::tcp_versioned(host, version)
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            Some(scheme) => Err(Error::UnsupportedScheme(scheme.to_string())),
            None => unreachable!(), // This is never possible because calling split on an empty string
                                    // always returns at least one element
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Creates a new podman instance for a podman host listening on a given Unix socket.
    ///
    /// `socket_path` is the part of URI that comes after the `unix://`. For example a URI `unix:///run/podman.sock` has a
    /// `socket_path` == "/run/podman.sock".
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Podman::unix_versioned`](Podman::unix_versioned).
    pub fn unix<P>(socket_path: P) -> Podman
    where
        P: AsRef<Path>,
    {
        Self::unix_versioned(socket_path, LATEST_API_VERSION)
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Same as [`Podman::unix`](Podman::unix) but the API version can be explicitly specified.
    pub fn unix_versioned<P>(socket_path: P, version: impl Into<ApiVersion>) -> Podman
    where
        P: AsRef<Path>,
    {
        Podman {
            version: version.into(),
            transport: Transport::Unix {
                client: Client::builder()
                    .pool_max_idle_per_host(0)
                    .build(get_unix_connector()),
                path: socket_path.as_ref().to_path_buf(),
            },
        }
    }

    #[cfg(feature = "tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
    /// Creates a new podman instance for a podman host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` or `https://` schemes,
    /// also known as authority part.
    ///
    /// `cert_path` specifies the base path in the filesystem containing a certificate (`cert.pem`)
    /// and a key (`key.pem`) that will be used by the client. If verify is `true` a CA file will be
    /// added (`ca.pem`) to the connector.
    ///
    /// Returns an error if the provided host will fail to parse as URL or reading the certificate
    /// files will fail.
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Podman::tls_versioned`](Podman::tls_versioned).
    pub fn tls<H, P>(host: H, cert_path: P, verify: bool) -> Result<Podman>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Self::tls_versioned(host, LATEST_API_VERSION, cert_path, verify)
    }

    #[cfg(feature = "tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
    /// Same as [`Podman::tls`](Podman::tls) but the API version can be explicitly specified.
    pub fn tls_versioned<H, P>(
        host: H,
        version: impl Into<ApiVersion>,
        cert_path: P,
        verify: bool,
    ) -> Result<Podman>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Ok(Podman {
            version: version.into(),
            transport: Transport::EncryptedTcp {
                client: Client::builder().build(get_https_connector(cert_path.as_ref(), verify)?),
                host: url::url::Url::parse(&format!("https://{}", host.as_ref()))
                    .map_err(Error::InvalidUrl)?,
            },
        })
    }

    /// Creates a new podman instance for a podman host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` schemes, also known as
    /// authority part.
    ///
    /// TLS is supported with feature `tls` enabled through [Podman::tls](Podman::tls) constructor.
    ///
    /// Returns an error if the provided host will fail to parse as URL.
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Podman::tcp_versioned`](Podman::tcp_versioned).
    pub fn tcp<H>(host: H) -> Result<Podman>
    where
        H: AsRef<str>,
    {
        Self::tcp_versioned(host, LATEST_API_VERSION)
    }

    /// Same as [`Podman::tcp`](Podman::tcp) but the API version can be explicitly specified.
    pub fn tcp_versioned<H>(host: H, version: impl Into<ApiVersion>) -> Result<Podman>
    where
        H: AsRef<str>,
    {
        Ok(Podman {
            version: version.into(),
            transport: Transport::Tcp {
                client: Client::builder().build(get_http_connector()),
                host: url::url::Url::parse(&format!("tcp://{}", host.as_ref()))
                    .map_err(Error::InvalidUrl)?,
            },
        })
    }

    /// Verifies the API version returned by the server and adjusts the version used by this client
    /// in future requests.
    pub async fn adjust_api_version(&mut self) -> Result<()> {
        let server_version: Option<ApiVersion> = self
            .version()
            .await
            .map(|v| v.api_version.and_then(|v| v.parse().ok()))?;

        if let Some(version) = server_version {
            if version <= self.version {
                self.version = version;
            }
        }

        Ok(())
    }

    //####################################################################################################
    //
    // API handles
    //
    //####################################################################################################

    api_doc! {
    containers
    /// Returns a handle to podman containers that can be used to operate on them.
    |
    pub fn containers(&self) -> api::Containers {
        api::Containers::new(self.clone())
    }}

    api_doc! {
    exec
    /// Returns a handle to podman execs that can be used to operate on them.
    |
    pub fn execs(&self) -> api::Execs {
        api::Execs::new(self.clone())
    }}

    api_doc! {
    images
    /// Returns a handle to podman images that can be used to operate on them.
    |
    pub fn images(&self) -> api::Images {
        api::Images::new(self.clone())
    }}

    api_doc! {
    manifests
    /// Returns a handle to podman manifests that can be used to operate on them.
    |
    pub fn manifests(&self) -> api::Manifests {
        api::Manifests::new(self.clone())
    }}

    api_doc! {
    networks
    /// Returns a handle to podman networks that can be used to operate on them.
    |
    pub fn networks(&self) -> api::Networks {
        api::Networks::new(self.clone())
    }}

    api_doc! {
    pods
    /// Returns a handle to podman pods that can be used to operate on them.
    |
    pub fn pods(&self) -> api::Pods {
        api::Pods::new(self.clone())
    }}

    api_doc! {
    volumes
    /// Returns a handle to podman volumes that can be used to operate on them.
    |
    pub fn volumes(&self) -> api::Volumes {
        api::Volumes::new(self.clone())
    }}

    api_doc! {
    secrets
    /// Returns a handle to podman secrets that can be used to operate on them.
    |
    pub fn secrets(&self) -> api::Secrets {
        api::Secrets::new(self.clone())
    }}

    //####################################################################################################
    //
    // Libpod System api
    //
    //####################################################################################################

    api_doc! {
    System => InfoLibpod
    /// Returns information on the system and libpod configuration
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.info().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn info(&self) -> Result<models::Info> {
        self.get_json("/libpod/info").await
    }}

    api_doc! {
    System => Ping
    /// Return protocol information from libpod.
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.ping().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn ping(&self) -> Result<models::LibpodPingInfo> {
        self.get("/libpod/_ping")
            .await
            .and_then(|resp| models::LibpodPingInfo::try_from(resp.headers()))
    }}

    api_doc! {
    System => VersionLibpod
    /// Returns component version information.
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.version().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn version(&self) -> Result<models::VersionResponse> {
        self.get_json("/libpod/version").await
    }}

    api_doc! {
    System => DataUsageLibpod
    /// Return information about disk usage for containers, images, and volumes.
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.data_usage().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn data_usage(&self) -> Result<models::SystemDfReport> {
        self.get_json("/libpod/system/df").await
    }}

    api_doc! {
    System => PruneLibpod
    /// Prune unused data.
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.prune().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn prune(&self) -> Result<models::SystemPruneReport> {
        self.post_json("/libpod/system/prune", Payload::empty())
            .await
    }}

    api_doc! {
    System => EventsLibpod
    |
    /// Returns system events
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use futures_util::StreamExt;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let mut events = podman.events(&Default::default());
    ///
    ///     while let Some(event) = events.next().await {
    ///         match event {
    ///             Ok(event) => println!("{:?}", event),
    ///             Err(e) => eprintln!("{}", e),
    ///         }
    ///     }
    /// };
    /// ```
    pub fn events<'libpod>(
        &'libpod self,
        opts: &EventsOpts,
    ) -> impl Stream<Item = Result<models::Event>> + Unpin + 'libpod {
        let ep = url::construct_ep("/libpod/events", opts.serialize());
        let reader = Box::pin(
            self.stream_get(ep)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            futures_codec::FramedRead::new(reader, futures_codec::LinesCodec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }}

    api_doc! {
    Play => KubeLibpod
    /// Create and run pods based on a Kubernetes YAML file (pod or service kind).
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     let yaml = r#"
    ///     apiVersion: v1
    ///     kind: Pod
    ///     metadata:
    ///       name: youthfulwescoff
    ///     spec:
    ///       containers:
    ///       - image: docker.io/library/alpine:latest
    ///         name: youthfulwescoff
    ///         securityContext:
    ///           capabilities:
    ///             drop:
    ///             - CAP_MKNOD
    ///             - CAP_NET_RAW
    ///             - CAP_AUDIT_WRITE
    ///         stdin: true
    ///         tty: true
    ///     "#;
    ///
    ///     match podman.play_kubernetes_yaml(&Default::default(), yaml).await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn play_kubernetes_yaml(
        &self,
        opts: &PlayKubernetesYamlOpts,
        yaml: impl Into<String>,
    ) -> Result<models::PlayKubeReport> {
        let ep = url::construct_ep("/libpod/play/kube", opts.serialize());
        let yaml = yaml.into();
        self.post_json(&ep, Payload::Text(yaml)).await
    }}

    api_doc! {
    Play => KubeDownLibpod
    /// Tears down pods defined in a YAML file.
    ///
    /// Example:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.remove_kubernetes_pods().await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    |
    pub async fn remove_kubernetes_pods(&self) -> Result<models::PlayKubeReport> {
        self.delete_json("/libpod/play/kube").await
    }}

    pub(crate) async fn resource_exists(
        &self,
        resource: ApiResource,
        id: &crate::Id,
    ) -> Result<bool> {
        let ep = format!("/libpod/{}/{}/exists", resource.as_ref(), id);
        match self.get(&ep).await {
            Ok(_) => Ok(true),
            Err(e) => match e {
                crate::Error::Fault {
                    code: crate::conn::http::StatusCode::NOT_FOUND,
                    message: _,
                } => Ok(false),
                e => Err(e),
            },
        }
    }

    pub(crate) async fn generate_systemd_units(
        &self,
        opts: &SystemdUnitsOpts,
        id: &crate::Id,
    ) -> Result<serde_json::Value> {
        let ep = url::construct_ep(
            format!("/libpod/generate/{}/systemd", &id),
            opts.serialize(),
        );
        self.get_json(&ep).await
    }

    pub(crate) async fn generate_kube_yaml(&self, service: bool, id: &crate::Id) -> Result<String> {
        let opts = [("names", id.to_string()), ("service", service.to_string())];
        let ep = url::construct_ep("/libpod/generate/kube", Some(url::encoded_pairs(opts)));

        let body = self.get(&ep).await.map(|b| b.into_body())?;
        transport::body_to_string(body).await.map_err(Error::from)
    }

    //####################################################################################################
    //
    // Utility functions to make requests
    //
    //####################################################################################################

    pub(crate) async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(
                Method::GET,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(
                Method::GET,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn post<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await
            .map_err(Error::from)
    }

    #[allow(dead_code)]
    pub(crate) async fn post_headers<B>(
        &self,
        endpoint: &str,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                headers,
            )
            .await
            .map_err(Error::from)
    }

    #[allow(dead_code)]
    pub(crate) async fn put<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::PUT,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn post_json<B, T>(
        &self,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        let raw_string = self
            .transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    #[allow(dead_code)]
    pub(crate) async fn post_json_headers<'a, B, T>(
        &self,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        let raw_string = self
            .transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                headers,
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn delete(&self, endpoint: &str) -> Result<String> {
        self.transport
            .request_string(
                Method::DELETE,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(
                Method::DELETE,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    #[allow(dead_code)]
    pub(crate) async fn head_response(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(
                Method::HEAD,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map_err(Error::from)
    }

    /// Send a streaming post request.
    ///
    /// Use stream_post_into_values if the endpoint returns JSON values
    pub(crate) fn stream_post<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'a
    where
        B: Into<Body> + 'a,
    {
        self.transport
            .stream_chunks(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                headers,
            )
            .map_err(Error::from)
    }

    #[allow(dead_code)]
    /// Send a streaming post request.
    fn stream_json_post<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'a
    where
        B: Into<Body> + 'a,
    {
        self.transport
            .stream_json_chunks(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                headers,
            )
            .map_err(Error::from)
    }

    #[allow(dead_code)]
    /// Send a streaming post request that returns a stream of JSON values
    ///
    /// When a received chunk does not contain a full JSON reads more chunks from the stream
    pub(crate) fn stream_post_into<'a, B, T>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<T>> + 'a
    where
        B: Into<Body> + 'a,
        T: DeserializeOwned,
    {
        self.stream_json_post(endpoint, body, headers)
            .and_then(|chunk| async move {
                trace!("got chunk {:?}", chunk);
                let stream = futures_util::stream::iter(
                    serde_json::Deserializer::from_slice(&chunk)
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
                .map_err(Error::from);

                Ok(stream)
            })
            .try_flatten()
    }

    pub(crate) fn stream_get<'a>(
        &'a self,
        endpoint: impl AsRef<str> + Unpin + 'a,
    ) -> impl Stream<Item = Result<Bytes>> + 'a {
        self.transport
            .stream_chunks(
                Method::GET,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .map_err(Error::from)
    }

    pub(crate) fn stream_get_json<'a, T>(
        &'a self,
        endpoint: impl AsRef<str> + Unpin + 'a,
    ) -> impl Stream<Item = Result<T>> + 'a
    where
        T: DeserializeOwned,
    {
        self.stream_get(endpoint)
            .and_then(|chunk| async move {
                let stream = futures_util::stream::iter(
                    serde_json::Deserializer::from_slice(&chunk)
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
                .map_err(Error::from);

                Ok(stream)
            })
            .try_flatten()
    }

    #[allow(dead_code)]
    pub(crate) async fn stream_post_upgrade<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
    ) -> Result<impl AsyncRead + AsyncWrite + 'a>
    where
        B: Into<Body> + 'a,
    {
        self.transport
            .stream_upgrade(Method::POST, self.version.make_endpoint(endpoint), body)
            .await
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Podman};
    #[test]
    fn creates_correct_podman() {
        let d = Podman::new("tcp://127.0.0.1:80");
        d.unwrap();
        let d = Podman::new("http://127.0.0.1:80");
        d.unwrap();

        #[cfg(unix)]
        {
            let d = Podman::new("unix://127.0.0.1:80");
            d.unwrap();
        }
        #[cfg(not(unix))]
        {
            let d = Podman::new("unix://127.0.0.1:80");
            assert!(d.is_err());
            match d.unwrap_err() {
                Error::UnsupportedScheme(scheme) if &scheme == "unix" => {}
                e => panic!(r#"Expected Error::UnsupportedScheme("unix"), got {}"#, e),
            }
        }

        let d = Podman::new("rand://127.0.0.1:80");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if &scheme == "rand" => {}
            e => panic!(r#"Expected Error::UnsupportedScheme("rand"), got {}"#, e),
        }

        let d = Podman::new("invalid_uri");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if &scheme == "invalid_uri" => {}
            e => panic!(
                r#"Expected Error::UnsupportedScheme("invalid_uri"), got {}"#,
                e
            ),
        }
        let d = Podman::new("");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if scheme.is_empty() => {}
            e => panic!(r#"Expected Error::UnsupportedScheme(""), got {}"#, e),
        }
    }
}
