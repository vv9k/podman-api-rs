#![cfg(unix)] //temporary
mod common;

use common::{create_base_volume, init_runtime, opts};

#[tokio::test]
async fn volume_create_exists_remove() {
    let podman = init_runtime();

    let volume = create_base_volume(&podman, "test-create-volume", None).await;

    assert!(volume.exists().await.unwrap());
    assert!(volume.remove().await.is_ok());
    assert!(!volume.exists().await.unwrap());
    let volume = create_base_volume(&podman, "test-create-volume", None).await;
    assert!(volume.exists().await.unwrap());
    assert!(volume.delete().await.is_ok());
    assert!(!volume.exists().await.unwrap());
}

#[tokio::test]
async fn volume_inspect() {
    let podman = init_runtime();
    let volumes = podman.volumes();

    let volume_name = "test-inspect-volume";
    create_base_volume(&podman, volume_name, None).await;

    let volume = volumes.get(volume_name);

    let inspect_result = volume.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data.name.contains(&volume_name));
    assert!(volume.remove().await.is_ok());
}

#[tokio::test]
async fn volume_prune() {
    let podman = init_runtime();
    let volumes = podman.volumes();

    let name_a = "test-prune-volume";
    let name_b = "test-prune-volume2";

    let label_key = "test-prune";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::VolumeCreateOpts::builder()
        .labels([(label_key, value_a)])
        .name(name_a)
        .build();
    let opts_b = opts::VolumeCreateOpts::builder()
        .labels([(label_key, value_b)])
        .name(name_b)
        .build();

    create_base_volume(&podman, name_a, Some(opts_a.clone())).await;
    create_base_volume(&podman, name_b, Some(opts_b.clone())).await;
    let volume_a = volumes.get(name_a);
    let volume_b = volumes.get(name_b);
    assert!(volume_a.exists().await.unwrap());
    assert!(volume_b.exists().await.unwrap());

    let filter = opts::VolumePruneFilter::LabelKey(label_key.to_string());
    let prune_opts = opts::VolumePruneOpts::builder().filter([filter]).build();
    let prune_result = volumes.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == name_a));
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == name_b));
    assert!(!volume_a.exists().await.unwrap());
    assert!(!volume_b.exists().await.unwrap());

    create_base_volume(&podman, name_a, Some(opts_a.clone())).await;
    create_base_volume(&podman, name_b, Some(opts_b.clone())).await;
    let volume_a = volumes.get(name_a);
    let volume_b = volumes.get(name_b);
    assert!(volume_a.exists().await.unwrap());
    assert!(volume_b.exists().await.unwrap());

    let filter = opts::VolumePruneFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let prune_opts = opts::VolumePruneOpts::builder().filter([filter]).build();
    let prune_result = volumes.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == name_a));
    assert!(!prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == name_b));
    assert!(!volume_a.exists().await.unwrap());
    assert!(volume_b.exists().await.unwrap());

    let filter = opts::VolumePruneFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let prune_opts = opts::VolumePruneOpts::builder().filter([filter]).build();
    let prune_result = volumes.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == name_b));

    assert!(!volume_a.exists().await.unwrap());
    assert!(!volume_b.exists().await.unwrap());
}

#[tokio::test]
async fn volume_list() {
    let podman = init_runtime();
    let volumes = podman.volumes();

    let name_a = "test-list-volume";
    let name_b = "test-list-volume2";

    let label_key = "test-list";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::VolumeCreateOpts::builder()
        .labels([(label_key, value_a)])
        .name(name_a)
        .build();
    let opts_b = opts::VolumeCreateOpts::builder()
        .labels([(label_key, value_b)])
        .name(name_b)
        .build();

    create_base_volume(&podman, name_a, Some(opts_a.clone())).await;
    create_base_volume(&podman, name_b, Some(opts_b.clone())).await;
    let volume_a = volumes.get(name_a);
    let volume_b = volumes.get(name_b);

    let filter = opts::VolumeListFilter::LabelKey(label_key.to_string());
    let list_opts = opts::VolumeListOpts::builder().filter([filter]).build();
    let list_result = volumes.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data.iter().any(|data| data.name == name_a));
    assert!(list_data.iter().any(|data| data.name == name_b));

    let filter = opts::VolumeListFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let list_opts = opts::VolumeListOpts::builder().filter([filter]).build();
    let list_result = volumes.list(&list_opts).await;
    // This sometimes breaks when running all tests at the same time
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data.iter().any(|data| data.name == name_a));
    assert!(!list_data.iter().any(|data| data.name == name_b));

    let filter = opts::VolumeListFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let list_opts = opts::VolumeListOpts::builder().filter([filter]).build();
    let list_result = volumes.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(!list_data.iter().any(|data| data.name == name_a));
    assert!(list_data.iter().any(|data| data.name == name_b));

    let _ = volume_a.remove().await;
    let _ = volume_b.remove().await;
}
