mod common;

use common::init_runtime;
use podman_api::{ApiVersion, LATEST_API_VERSION};

#[tokio::test]
async fn podman_adjust_api_version() {
    // Negotiation must succeed against real servers and cap the client
    // version at min(client LATEST_API_VERSION, server version).
    let mut podman = init_runtime();
    podman
        .adjust_api_version()
        .await
        .expect("adjust api version");

    // After negotiation, requests keep working at the negotiated version.
    let info = podman.version().await.expect("version after negotiation");
    let server: ApiVersion = info
        .version
        .expect("server reports version")
        .parse()
        .expect("server version parses");
    assert!(
        server >= LATEST_API_VERSION,
        "server {server} < {LATEST_API_VERSION}"
    );
    assert!(podman.ping().await.is_ok());
    assert!(podman.containers().list(&Default::default()).await.is_ok());
}

#[tokio::test]
async fn podman_info() {
    let podman = init_runtime();

    let info_result = podman.info().await;
    assert!(info_result.is_ok());
    let info_data = info_result.unwrap();
    assert_eq!(
        info_data.host.as_ref().unwrap().hostname.as_ref().unwrap(),
        &gethostname::gethostname().into_string().unwrap()
    );
}

#[tokio::test]
async fn podman_ping() {
    let podman = init_runtime();

    let ping_result = podman.ping().await;
    assert!(ping_result.is_ok());
    let ping_data = ping_result.unwrap();
    assert!(!ping_data.api_version.is_empty());
}

#[tokio::test]
async fn podman_version() {
    let podman = init_runtime();

    let version_result = podman.version().await;
    assert!(version_result.is_ok());
    let version_data = version_result.unwrap();

    let ping_result = podman.ping().await;
    assert!(ping_result.is_ok());
    let ping_data = ping_result.unwrap();

    assert_eq!(ping_data.api_version, version_data.api_version.unwrap());
}

#[tokio::test]
async fn podman_data_usage() {
    let podman = init_runtime();

    let du_result = podman.data_usage().await;
    assert!(du_result.is_ok());
    let _du_data = du_result.unwrap();
}
