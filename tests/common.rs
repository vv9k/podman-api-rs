#![cfg(unix)] //temporary
#![allow(dead_code)]
use std::env;

pub use futures_util::{StreamExt, TryStreamExt};
pub use podman_api::{api, conn, models, opts, Podman};
pub use tempdir::TempDir;

pub const DEFAULT_IMAGE: &str = "ubuntu:latest";
pub const DEFAULT_CMD: &str = "sleep inf";
pub const DEFAULT_CMD_ARRAY: &[&str] = &["sleep", "inf"];
pub const TEST_IMAGE_PATH: &str = "/var/test123";

pub fn init_runtime() -> Podman {
    let _ = env_logger::try_init();
    let podman_uri = env::var("PODMAN_API_URI").expect("podman socket location PODMAN_API_URI");

    Podman::new(podman_uri).unwrap()
}

pub async fn create_base_container(
    podman: &Podman,
    name: &str,
    opts: Option<opts::ContainerCreateOpts>,
) -> api::Container {
    cleanup_container(podman, name).await;

    let opts = opts.unwrap_or_else(|| {
        opts::ContainerCreateOpts::builder()
            .name(name)
            .image(DEFAULT_IMAGE)
            .command(DEFAULT_CMD_ARRAY)
            .build()
    });
    podman
        .containers()
        .create(&opts)
        .await
        .expect("created base container");
    podman.containers().get(name)
}

pub async fn cleanup_container(podman: &Podman, name: &str) {
    let _ = podman.containers().get(name).remove().await;
}

pub async fn get_container_full_id(podman: &Podman, name: &str) -> String {
    podman
        .containers()
        .get(name)
        .inspect()
        .await
        .map(|data| data.id)
        .expect("container inspect data")
        .expect("container full id")
}

pub fn tempdir_with_dockerfile(name: &str, content: Option<&str>) -> TempDir {
    let tmp = TempDir::new(name).expect("temp dir for image");
    let default_dockerfile = format!(
        "FROM {DEFAULT_IMAGE}\nRUN echo 1234 > {TEST_IMAGE_PATH}\nRUN echo 321\nCMD sleep inf",
    );

    std::fs::write(
        tmp.path().join("Dockerfile"),
        content.unwrap_or(default_dockerfile.as_str()),
    )
    .expect("saved Dockerfile");
    tmp
}

pub async fn create_base_image(
    podman: &Podman,
    tag: &str,
    opts: Option<opts::ImageBuildOpts>,
) -> api::Image {
    let images = podman.images();
    let _ = images.get(tag).remove().await;

    let tmp = tempdir_with_dockerfile(tag, None);

    let opts = opts.unwrap_or_else(|| {
        opts::ImageBuildOpts::builder(tmp.path().to_string_lossy())
            .tag(tag)
            .build()
    });

    let mut image_stream = images.build(&opts).expect("image build stream");
    let mut last = None;
    while let Some(chunk) = image_stream.next().await {
        assert!(chunk.is_ok());
        last = Some(chunk);
    }

    podman
        .images()
        .get(last.unwrap().unwrap().stream.trim_end().to_string())
}

pub async fn get_image_full_id(podman: &Podman, name: &str) -> String {
    podman
        .images()
        .get(name)
        .inspect()
        .await
        .map(|data| data.id)
        .expect("image inspect data")
        .expect("image full id")
}

pub async fn create_base_volume(
    podman: &Podman,
    name: &str,
    opts: Option<opts::VolumeCreateOpts>,
) -> api::Volume {
    cleanup_volume(podman, name).await;

    let opts = opts.unwrap_or_else(|| opts::VolumeCreateOpts::builder().name(name).build());
    podman
        .volumes()
        .create(&opts)
        .await
        .expect("created base volume");
    podman.volumes().get(name)
}

pub async fn cleanup_volume(podman: &Podman, name: &str) {
    let _ = podman.volumes().get(name).remove().await;
}

pub async fn create_base_network(
    podman: &Podman,
    name: &str,
    opts: Option<opts::NetworkCreateOpts>,
) -> api::Network {
    cleanup_network(podman, name).await;

    let opts = opts.unwrap_or_else(|| opts::NetworkCreateOpts::builder().name(name).build());
    podman
        .networks()
        .create(&opts)
        .await
        .expect("created base network");
    podman.networks().get(name)
}

pub async fn cleanup_network(podman: &Podman, name: &str) {
    let _ = podman.networks().get(name).remove().await;
}
