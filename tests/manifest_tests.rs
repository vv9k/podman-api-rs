#![cfg(unix)] //temporary
mod common;

use common::{api::Manifest, init_runtime, opts, Podman, DEFAULT_IMAGE};

//####################################################################################################

async fn create_base_manifest(
    podman: &Podman,
    name: &str,
    opts: Option<opts::ManifestCreateOpts>,
) -> Manifest {
    let manifests = podman.manifests();
    let _ = manifests.get(name).delete().await;

    let opts = opts.unwrap_or_else(|| {
        opts::ManifestCreateOpts::builder(name)
            .images([DEFAULT_IMAGE])
            .build()
    });

    manifests.create(&opts).await.expect("created manifest")
}

//####################################################################################################

#[tokio::test]
async fn manifest_create_exists_remove() {
    let podman = init_runtime();
    let manifest_name = "test-create-manifest";

    assert!(!podman
        .manifests()
        .get(manifest_name)
        .exists()
        .await
        .unwrap());
    let manifest = create_base_manifest(&podman, manifest_name, None).await;
    assert!(manifest.exists().await.unwrap());
    assert!(manifest.delete().await.is_ok());
    assert!(!manifest.exists().await.unwrap());
}

#[tokio::test]
async fn manifest_inspect() {
    let podman = init_runtime();

    let manifest_name = "test-inspect-manifest";
    let manifest = create_base_manifest(&podman, manifest_name, None).await;

    let inspect_result = manifest.inspect().await;
    assert!(inspect_result.is_ok());
    assert!(manifest.delete().await.is_ok());
}

#[tokio::test]
async fn manifest_add_image() {
    let podman = init_runtime();
    let manifest_name = "test-add-image-manifest";

    let manifest = create_base_manifest(&podman, manifest_name, None).await;
    let opts = opts::ManifestImageAddOpts::builder()
        .all(true)
        .images(["docker.io/library/alpine:latest"])
        .build();
    let add_result = manifest.add_image(&opts).await;
    assert!(add_result.is_ok());
    let new_id = add_result.unwrap().id;
    let new_manifest = podman.manifests().get(new_id);
    let new_data = new_manifest.inspect().await;
    assert!(new_data.is_ok());
    let _ = manifest.delete().await;
    let _ = new_manifest.delete().await;
}

#[tokio::test]
async fn manifest_remove_image() {
    let podman = init_runtime();
    let manifest_name = "test-remove-image-manifest";

    let manifest = create_base_manifest(&podman, manifest_name, None).await;
    let opts = opts::ManifestImageAddOpts::builder()
        .all(true)
        .images(["docker.io/library/alpine:latest"])
        .build();
    let add_result = manifest.add_image(&opts).await;
    assert!(add_result.is_ok());
    let new_id = add_result.unwrap().id;
    let new_manifest = podman.manifests().get(new_id);
    let inspect_data = new_manifest.inspect().await.expect("manifest inspect data");
    let digest = inspect_data.manifests.as_ref().unwrap()[0]
        .digest
        .as_ref()
        .unwrap();

    let remove_result = manifest.remove_image(digest).await;
    assert!(remove_result.is_ok());
    assert!(remove_result
        .unwrap()
        .untagged
        .unwrap()
        .iter()
        .any(|it| it.contains(manifest_name)));

    let _ = manifest.delete().await;
    let _ = new_manifest.delete().await;
}
