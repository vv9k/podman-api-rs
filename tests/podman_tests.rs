#![cfg(unix)] //temporary
mod common;

use common::init_runtime;

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
