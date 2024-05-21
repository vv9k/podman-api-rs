use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_filter_func, impl_map_field, impl_opts_builder, impl_opts_required_builder,
    impl_url_bool_field, impl_url_enum_field, impl_url_field, impl_url_str_field,
    impl_url_vec_field,
};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

impl_opts_required_builder!(url =>
    /// Adjust how an image is built.
    ImageBuild,
    ///
    /// Parameters:
    /// * path - Path to a build context directory
    path => "path"
);

#[derive(Debug, Clone, PartialEq, Eq)]
/// The networking mode for the run commands during image build.
#[derive(Default)]
pub enum NetworkMode {
    /// Limited to containers within a single host, port mapping required for external access.
    #[default]
    Bridge,
    /// No isolation between host and containers on this network.
    Host,
    /// Disable all networking for this container.
    None,
    /// Share networking with given container.
    Container,
    /// Custom network's name.
    Custom(String),
}

impl AsRef<str> for NetworkMode {
    fn as_ref(&self) -> &str {
        match self {
            NetworkMode::Bridge => "bridge",
            NetworkMode::Host => "host",
            NetworkMode::None => "none",
            NetworkMode::Container => "container",
            NetworkMode::Custom(custom) => custom,
        }
    }
}

impl fmt::Display for NetworkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Used to set the image platform with [`platform`](ImageBuildOptsBuilder::platform).
pub struct Platform {
    os: String,
    arch: Option<String>,
    version: Option<String>,
}

impl Platform {
    pub fn new(os: impl Into<String>) -> Self {
        Self {
            os: os.into(),
            arch: None,
            version: None,
        }
    }

    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = Some(arch.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(arch) = &self.arch {
            if let Some(vers) = &self.version {
                write!(f, "{}/{}/{}", self.os, arch, vers)
            } else {
                write!(f, "{}/{}", self.os, arch)
            }
        } else {
            write!(f, "{}", self.os)
        }
    }
}

impl ImageBuildOptsBuilder {
    impl_url_bool_field!(
        /// Instead of building for a set of platforms specified using the platform option,
        /// inspect the build's base images, and build for all of the platforms that are available.
        /// Stages that use scratch as a starting point can not be inspected, so at least one
        /// non-scratch stage must be present for detection to work usefully.
        all_platforms => "allplatforms"
    );

    impl_map_field!(url
        /// Key-value build time variables.
        build_args => "buildargs"
    );

    /// List of images used to build cache resolution
    pub fn cache_from<S>(mut self, images: impl IntoIterator<Item = S>) -> Self
    where
        S: Into<String>,
    {
        self.params.insert(
            "cachefrom",
            serde_json::to_string(&images.into_iter().map(|i| i.into()).collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        self
    }

    impl_url_field!(
        /// Limits the CPU CFS (Completely Fair Scheduler) period.
        cpu_period: isize => "cpuperiod"
    );

    impl_url_field!(
        /// Limits the CPU CFS (Completely Fair Scheduler) quota.
        cpu_quota: isize => "cpuquota"
    );

    impl_url_field!(
        /// Set CPUs in which to allow execution. Example: `0-1`, `1-3`
        cpu_set_cpus: isize => "cpusetcpus"
    );

    impl_url_field!(
        /// CPU shares - relative weights
        cpu_shares: isize => "cpushares"
    );

    impl_url_str_field!(
        /// Path within the build context to the Dockerfile. This is ignored
        /// if remote is specified and points to an external Dockerfile.
        dockerfile => "dockerfile"
    );

    impl_url_str_field!(
        /// Extra hosts to add to /etc/hosts.
        extra_hosts => "extrahosts"
    );

    impl_url_bool_field!(
        /// Always remove intermediate containers, even upon failure.
        force_rm => "forcerm"
    );

    impl_url_bool_field!(
        /// Inject http proxy environment variables into container.
        http_proxy => "httpproxy"
    );

    impl_map_field!(url
        /// Key-value pairs to set as labels on the new image.
        labels => "labels"
    );

    impl_url_bool_field!(
        /// Cache intermediate layers during build.
        layers => "layers"
    );

    impl_url_field!(
        /// The upper limit (in bytes) on how much memory running
        /// containers can use.
        memory: usize => "memory"
    );

    impl_url_field!(
        /// Limits the amount of memory and swap together.
        memswap: usize => "memswap"
    );

    impl_url_enum_field!(
        /// Set the networking mode for the run commands during build.
        network_mode: NetworkMode => "networkmode"
    );

    impl_url_bool_field!(
        /// Do not use the cache when building the image.
        no_cache => "nocache"
    );

    impl_url_str_field!(
        /// Output configuration.
        outputs => "outputs"
    );

    pub fn platform(mut self, platform: Platform) -> Self {
        self.params.insert("platform", platform.to_string());
        self
    }

    impl_url_bool_field!(
        /// Attempt to pull the image even if an older image exists locally.
        pull => "pull"
    );

    impl_url_bool_field!(
        /// Suppress verbose build output.
        quiet => "q"
    );

    impl_url_str_field!(
        /// A Git repository URI or HTTP/HTTPS context URI. If the URI points
        /// to a single text file, the file’s contents are placed into a file
        /// called Dockerfile and the image is built from that file. If the URI
        /// points to a tarball, the file is downloaded by the daemon and
        /// the contents therein used as the context for the build. If the URI
        /// points to a tarball and the dockerfile parameter is also specified,
        /// there must be a file with the corresponding path inside the tarball.
        remote => "remote"
    );

    impl_url_bool_field!(
        /// Remove intermediate containers after a successful build.
        remove => "rm"
    );

    impl_url_field!(
        /// Value to use when mounting an shmfs on the container's /dev/shm directory.
        /// Default is 64MB
        shared_mem_size: usize => "shmsize"
    );

    impl_url_bool_field!(
        /// Silently ignored. Squash the resulting images layers into a single layer.
        squash => "squash"
    );

    impl_url_str_field!(
        /// A name and optional tag to apply to the image in the `name:tag` format.
        tag => "t"
    );

    impl_url_str_field!(
        /// Target build stage
        target => "target"
    );

    impl_url_vec_field!(
        /// Unset environment variables from the final image.
        unset_env => "unsetenv"
    );
}

impl_opts_builder!(url =>
    /// Adjust how images are listed.
    ImageList
);

#[derive(Debug, Clone)]
/// Used to filter listed images with [`ImagesListFilter`](ImagesListFilter).
pub enum ImageOpt {
    Name(crate::Id),
    Tag(crate::Id, String),
    Digest(crate::Id, String),
}

impl fmt::Display for ImageOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImageOpt::*;
        match self {
            Name(id) => write!(f, "{id}"),
            Tag(id, tag) => write!(f, "{id}:{tag}"),
            Digest(id, digest) => write!(f, "{id}@{digest}"),
        }
    }
}

#[derive(Debug)]
/// Used to filter listed images by one of the variants.
pub enum ImageListFilter {
    Before(ImageOpt),
    Dangling(bool),
    /// Image with key label.
    LabelKey(String),
    /// Image with key-value label.
    LabelKeyVal(String, String),
    /// Image without key label.
    NoLabelKey(String),
    /// Image without key=value label.
    NoLabelKeyVal(String, String),
    /// Image name with optional tag.
    Reference(crate::Id, Option<String>),
    Id(crate::Id),
    Since(ImageOpt),
}

impl Filter for ImageListFilter {
    fn query_item(&self) -> FilterItem {
        use ImageListFilter::*;
        match &self {
            Before(image) => FilterItem::new("before", image.to_string()),
            Dangling(dangling) => FilterItem::new("dangling", dangling.to_string()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
            Reference(image, tag) => FilterItem::new(
                "reference",
                if let Some(tag) = tag {
                    format!("{image}:{tag}")
                } else {
                    image.to_string()
                },
            ),
            Id(id) => FilterItem::new("id", id.to_string()),
            Since(image) => FilterItem::new("since", image.to_string()),
        }
    }
}

impl ImageListOptsBuilder {
    impl_filter_func!(ImageListFilter);

    impl_url_bool_field!(
        /// Show all images. Only images from a final layer (no children) are shown by default.
        all => "all"
    );
}

impl_opts_builder!(url =>
    /// Adjust the way an image is tagged/untagged.
    ImageTag
);

impl ImageTagOptsBuilder {
    impl_url_str_field!(
        /// Set the image repository.
        repo => "repo"
    );

    impl_url_str_field!(
        /// Set the image tag.
        tag => "tag"
    );
}

#[derive(Clone, Serialize, Debug)]
#[serde(untagged)]
pub enum RegistryAuth {
    Password {
        username: String,
        password: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,

        #[serde(rename = "serveraddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        server_address: Option<String>,
    },
    Token {
        #[serde(rename = "identitytoken")]
        identity_token: String,
    },
}

impl RegistryAuth {
    /// return a new instance with token authentication
    pub fn token(token: impl Into<String>) -> RegistryAuth {
        RegistryAuth::Token {
            identity_token: token.into(),
        }
    }

    /// return a new instance of a builder for authentication
    pub fn builder() -> RegistryAuthBuilder {
        RegistryAuthBuilder::default()
    }

    /// serialize authentication as JSON in base64
    pub fn serialize(&self) -> String {
        serde_json::to_string(self)
            .map(|c| base64::encode_config(c, base64::URL_SAFE))
            .unwrap_or_default()
    }
}

#[derive(Default)]
pub struct RegistryAuthBuilder {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    server_address: Option<String>,
}

impl RegistryAuthBuilder {
    /// The username used for authentication.
    pub fn username(&mut self, username: impl Into<String>) -> &mut Self {
        self.username = Some(username.into());
        self
    }

    /// The password used for authentication.
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = Some(password.into());
        self
    }

    /// The email addres used for authentication.
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    /// The server address of registry, should be a domain/IP without a protocol.
    /// Example: `10.92.0.1`, `docker.corp.local`
    pub fn server_address(&mut self, server_address: impl Into<String>) -> &mut Self {
        self.server_address = Some(server_address.into());
        self
    }

    /// Create the final authentication object.
    pub fn build(&self) -> RegistryAuth {
        RegistryAuth::Password {
            username: self.username.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default(),
            email: self.email.clone(),
            server_address: self.server_address.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct PullOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl PullOpts {
    /// return a new instance of a builder for Opts
    pub fn builder() -> PullOptsBuilder {
        PullOptsBuilder::default()
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(containers_api::url::encoded_pairs(self.params.iter()))
        }
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The networking mode for the run commands during image build.
#[derive(Default)]
pub enum PullPolicy {
    #[default]
    Always,
    Missing,
    Newer,
    Never,
}

impl AsRef<str> for PullPolicy {
    fn as_ref(&self) -> &str {
        match self {
            PullPolicy::Always => "always",
            PullPolicy::Missing => "missing",
            PullPolicy::Newer => "newer",
            PullPolicy::Never => "never",
        }
    }
}

impl fmt::Display for PullPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Default)]
pub struct PullOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl PullOptsBuilder {
    impl_url_bool_field!(
        /// Pull all tagged images in the repository.
        all_tags => "allTags"
    );

    impl_url_str_field!(
        /// Pull image for the specified architecture.
        arch => "Arch"
    );

    impl_url_str_field!(
        /// username:password for the registry.
        credentials => "credentials"
    );

    impl_url_str_field!(
        /// Pull image for the specified operating system.
        os => "OS"
    );

    impl_url_enum_field!(
        /// Image pull policy.
        policy: PullPolicy => "policy"
    );

    impl_url_bool_field!(
        /// Silences extra stream data on pull.
        quiet => "quiet"
    );

    impl_url_str_field!(
        /// Mandatory reference to the image.
        reference => "reference"
    );

    impl_url_bool_field!(
        /// Require TLS verification.
        tls_verify => "tlsVerify"
    );

    impl_url_str_field!(
        /// Pull image for the specified variant.
        variant => "Variant"
    );

    pub fn auth(&mut self, auth: RegistryAuth) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(&mut self) -> PullOpts {
        PullOpts {
            auth: self.auth.take(),
            params: self.params.clone(),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how an image is exported.
    ImageExport
);

impl ImageExportOptsBuilder {
    impl_url_bool_field!(
        /// Use compression on image.
        compress => "compress"
    );

    impl_url_str_field!(
        /// Format for exported image.
        format => "format"
    );
}

impl_opts_builder!(url =>
    /// Adjust how an image is imported.
    ImageImport
);

impl ImageImportOptsBuilder {
    impl_url_vec_field!(
        /// Apply the following possible instructions to the created image: CMD | ENTRYPOINT | ENV | EXPOSE | LABEL | STOPSIGNAL | USER | VOLUME | WORKDIR.
        changes => "changes"
    );

    impl_url_str_field!(
        /// Set commit message for imported image.
        message => "message"
    );

    impl_url_str_field!(
        /// Optional Name[:TAG] for the image.
        reference => "reference"
    );

    impl_url_str_field!(
        /// Load image from the specified URL.
        url => "url"
    );
}

impl_opts_builder!(url =>
    /// Adjust how the image tree is retrieved.
    ImageTree
);

impl ImageTreeOptsBuilder {
    impl_url_bool_field!(
        /// Show all child images and layers of the specified image.
        what_requires => "whatrequires"
    );
}

impl_opts_builder!(url =>
    /// Adjust how multiple images will be removed.
    ImagesRemove
);

impl ImagesRemoveOptsBuilder {
    impl_url_bool_field!(
        /// Remove all images.
        all => "all"
    );

    impl_url_bool_field!(
        /// Force image removal (including containers using the images).
        force => "force"
    );

    impl_url_bool_field!(
        /// Ignore if a specified image does not exist and do not throw an error.
        ignore => "ignore"
    );

    impl_url_vec_field!(
        /// Images IDs or names to remove.
        images => "images"
    );

    impl_url_bool_field!(
        /// Resolves to manifest list instead of image.
        lookup_manifest => "lookupManifest"
    );
}

#[derive(Default, Debug)]
/// Adjust how an image is pushed to a registry.
pub struct ImagePushOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl ImagePushOpts {
    /// Return a new instance of a builder for ImagePushOpts.
    pub fn builder() -> ImagePushOptsBuilder {
        ImagePushOptsBuilder::default()
    }

    /// Serialize ImagePushOpts as a string. Returns None if no Opts are defined.
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(containers_api::url::encoded_pairs(self.params.iter()))
        }
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

#[derive(Debug, Default)]
pub struct ImagePushOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl ImagePushOptsBuilder {
    impl_url_str_field!(
        /// Allows for pushing the image to a different destination than the image refers to.
        destination => "destination"
    );

    impl_url_bool_field!(
        /// Silences extra stream data on push.
        quiet => "quiet"
    );

    impl_url_bool_field!(
        /// Require TLS verification.
        tls_verify => "tlsVerify"
    );

    pub fn auth(mut self, auth: RegistryAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> ImagePushOpts {
        ImagePushOpts {
            auth: self.auth,
            params: self.params,
        }
    }
}

#[derive(Debug)]
/// Used to filter removed images.
pub enum ImagePruneFilter {
    /// When set to true, prune only unused and untagged images. When set to false, all unused
    /// images are pruned.
    Dangling(bool),
    /// Prune images created before this timestamp. The <timestamp> can be Unix timestamps, date
    /// formatted timestamps, or Go duration strings (e.g. 10m, 1h30m) computed relative to the
    /// daemon machine’s time.
    // #TODO: DateTime
    Until(String),
    /// Image with key label.
    LabelKey(String),
    /// Image with key-value label.
    LabelKeyVal(String, String),
    /// Image without key label.
    NoLabelKey(String),
    /// Image without key-value label.
    NoLabelKeyVal(String, String),
}

impl Filter for ImagePruneFilter {
    fn query_item(&self) -> FilterItem {
        use ImagePruneFilter::*;
        match &self {
            Dangling(dangling) => FilterItem::new("dangling", dangling.to_string()),
            Until(until) => FilterItem::new("until", until.to_string()),
            LabelKey(key) => FilterItem::new("label", key.clone()),
            LabelKeyVal(key, val) => FilterItem::new("label", format!("{key}={val}")),
            NoLabelKey(key) => FilterItem::new("label!", key.clone()),
            NoLabelKeyVal(key, val) => FilterItem::new("label!", format!("{key}={val}")),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how unused images are removed.
    ImagePrune
);

impl ImagePruneOptsBuilder {
    impl_filter_func!(
        /// Filters to apply to image pruning.
        ImagePruneFilter
    );

    impl_url_bool_field!(
        /// Remove all images not in use by containers, not just dangling ones.
        all => "all"
    );

    impl_url_bool_field!(
        /// Remove images even when they are used by external containers (e.g, by build
        /// containers).
        external => "external"
    );
}

#[derive(Debug)]
/// Used to filter searched images.
pub enum ImageSearchFilter {
    IsAutomated(bool),
    IsOfficial(bool),
    /// Matches images that has at least 'number' stars.
    Stars(usize),
}

impl Filter for ImageSearchFilter {
    fn query_item(&self) -> FilterItem {
        use ImageSearchFilter::*;
        match &self {
            IsAutomated(is_automated) => FilterItem::new("is-automated", is_automated.to_string()),
            IsOfficial(is_official) => FilterItem::new("is-official", is_official.to_string()),
            Stars(stars) => FilterItem::new("stars", stars.to_string()),
        }
    }
}

impl_opts_builder!(url =>
    /// Adjust how to search for images in registries.
    ImageSearch
);

impl ImageSearchOptsBuilder {
    impl_filter_func!(
        /// Filters to process on the images list.
        ImageSearchFilter
    );

    impl_url_field!(
        /// Maximum number of results.
        limit: usize => "limit"
    );

    impl_url_bool_field!(
        /// List the available tags in the repository.
        list_tags => "listTags"
    );

    impl_url_str_field!(
        /// Term to search for.
        term => "term"
    );

    impl_url_bool_field!(
        /// Skip TLS verification for registries.
        tls_verify => "tlsVerify"
    );
}

impl_opts_builder!(url =>
    /// Adjust how multiple images are exported.
    ImagesExport
);

impl ImagesExportOptsBuilder {
    impl_url_bool_field!(
        /// Use compression on image.
        compress => "compress"
    );

    impl_url_str_field!(
        /// Format for exported image (only docker-archive is supported).
        format => "format"
    );

    impl_url_bool_field!(
        /// Accept uncompressed layers when copying OCI images.
        oci_accept_uncompressed_layers => "ociAcceptUncompressedLayers"
    );

    impl_url_vec_field!(
        /// References to images to export.
        references => "references"
    );
}
