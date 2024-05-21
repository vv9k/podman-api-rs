mod common;

use common::{
    create_base_image, get_image_full_id, init_runtime, models, opts, tempdir_with_dockerfile,
    TryStreamExt, DEFAULT_IMAGE, TEST_IMAGE_PATH,
};

#[tokio::test]
async fn image_create_exists_remove() {
    let podman = init_runtime();

    let image = create_base_image(&podman, "test-create-image", None).await;
    assert!(image.exists().await.unwrap());
    assert!(image.remove().await.is_ok());
    assert!(!image.exists().await.unwrap());
}

#[tokio::test]
async fn image_inspect() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-inspect-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let inspect_result = image.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data
        .names_history
        .as_ref()
        .unwrap()
        .contains(&format!("localhost/{image_name}:latest")));
    assert!(image.remove().await.is_ok());
}

#[tokio::test]
async fn image_history() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-history-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let history_result = image.history().await;
    assert!(history_result.is_ok());
    let history_data = history_result.unwrap();
    assert!(history_data
        .iter()
        .any(|item| item.comment.as_ref().unwrap().contains(DEFAULT_IMAGE)));
}

#[tokio::test]
async fn image_changes() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-changes-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let changes_result = image.changes(&Default::default()).await;
    assert!(changes_result.is_ok());
    let changes_data = changes_result.unwrap();
    assert!(changes_data.contains(&models::FilesystemChange {
        kind: 1,
        path: TEST_IMAGE_PATH.into()
    }));

    //cleanup
    let _ = image.remove().await;
}

#[tokio::test]
async fn image_tree() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-tree-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let tree_result = image.tree(&Default::default()).await;
    assert!(tree_result.is_ok());
    let tree_data = tree_result.unwrap();
    assert!(tree_data.tree.as_ref().unwrap().contains(image_name));

    //cleanup
    let _ = image.remove().await;
}

#[tokio::test]
async fn image_tag_untag() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-tag-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let opts = opts::ImageTagOpts::builder()
        .repo(image_name)
        .tag("1.0.0")
        .build();

    assert!(image.tag(&opts).await.is_ok());

    let new_tag = format!("localhost/{image_name}:1.0.0");

    assert!(image
        .inspect()
        .await
        .expect("image inspect data")
        .repo_tags
        .expect("repo tags")
        .contains(&new_tag));

    assert!(image.untag(&opts).await.is_ok());

    // without the timeout there is an error:
    //  500 runtime error: invalid memory address or nil pointer dereference
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    assert!(!image
        .inspect()
        .await
        .expect("image inspect data")
        .repo_tags
        .expect("repo tags")
        .contains(&new_tag));

    //cleanup
    let _ = image.remove().await;
}

#[tokio::test]
async fn image_export_import() {
    let podman = init_runtime();
    let images = podman.images();

    let image_name = "test-export-image";
    create_base_image(&podman, image_name, None).await;

    let image = images.get(image_name);

    let export_stream = image.export(&Default::default());
    let export_data = export_stream.try_concat().await.expect("image archive");
    assert!(!export_data.is_empty());

    let _ = image.remove().await;

    assert!(!image.exists().await.unwrap());

    let import_opts = opts::ImageImportOpts::builder()
        .reference(image_name)
        .build();
    let import_result = images.import(&import_opts, &export_data).await;
    assert!(import_result.is_ok());
    assert!(image.exists().await.unwrap());
    let full_id = get_image_full_id(&podman, image_name).await;

    let export_opts = opts::ImagesExportOpts::builder()
        .references([full_id])
        .build();
    let export_stream = images.export(&export_opts);
    let export_data = export_stream.try_concat().await.expect("image archive");
    assert!(!export_data.is_empty());

    let _ = image.remove().await;
    assert!(!image.exists().await.unwrap());

    let import_result = images.import(&import_opts, &export_data).await;
    assert!(import_result.is_ok());
    assert!(image.exists().await.unwrap());

    let _ = image.remove().await;
    assert!(!image.exists().await.unwrap());
}

#[tokio::test]
async fn image_search() {
    let podman = init_runtime();
    let images = podman.images();

    let opts = opts::ImageSearchOpts::builder().term("ubuntu").build();

    let search_result = images.search(&opts).await;
    assert!(search_result.is_ok());
    //let search_data = search_result.unwrap();
    //log::error!("{search_data:#?}");
}

#[tokio::test]
async fn image_prune() {
    let podman = init_runtime();
    let images = podman.images();

    let name_a = "test-prune-image";
    let name_b = "test-prune-image2";

    let tmp = tempdir_with_dockerfile(name_a, None);

    let label_key = "test-prune";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::ImageBuildOpts::builder(tmp.path().to_string_lossy())
        .labels([(label_key, value_a)])
        .tag(name_a)
        .build();
    let opts_b = opts::ImageBuildOpts::builder(tmp.path().to_string_lossy())
        .labels([(label_key, value_b)])
        .tag(name_b)
        .build();

    create_base_image(&podman, name_a, Some(opts_a.clone())).await;
    create_base_image(&podman, name_b, Some(opts_b.clone())).await;
    let image_a = images.get(name_a);
    let image_b = images.get(name_b);
    let full_id_a = get_image_full_id(&podman, name_a).await;
    let full_id_b = get_image_full_id(&podman, name_b).await;
    assert!(image_a.exists().await.unwrap());
    assert!(image_b.exists().await.unwrap());

    let filter = opts::ImagePruneFilter::LabelKey(label_key.to_string());
    let prune_opts = opts::ImagePruneOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let prune_result = images.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data.is_some());
    let prune_data = prune_data.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_a));
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_b));
    assert!(!image_a.exists().await.unwrap());
    assert!(!image_b.exists().await.unwrap());

    create_base_image(&podman, name_a, Some(opts_a.clone())).await;
    create_base_image(&podman, name_b, Some(opts_b.clone())).await;
    let image_a = images.get(name_a);
    let image_b = images.get(name_b);
    let full_id_a = get_image_full_id(&podman, name_a).await;
    let full_id_b = get_image_full_id(&podman, name_b).await;
    assert!(image_a.exists().await.unwrap());
    assert!(image_b.exists().await.unwrap());

    let filter = opts::ImagePruneFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let prune_opts = opts::ImagePruneOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let prune_result = images.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data.is_some());
    let prune_data = prune_data.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_a));
    assert!(!prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_b));
    assert!(!image_a.exists().await.unwrap());
    assert!(image_b.exists().await.unwrap());

    let filter = opts::ImagePruneFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let prune_opts = opts::ImagePruneOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let prune_result = images.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data.is_some());
    let prune_data = prune_data.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_b));

    assert!(!image_a.exists().await.unwrap());
    assert!(!image_b.exists().await.unwrap());
}

#[tokio::test]
async fn image_list() {
    let podman = init_runtime();
    let images = podman.images();

    let name_a = "test-list-image";
    let name_b = "test-list-image2";

    let tmp = tempdir_with_dockerfile(name_a, None);

    let label_key = "test-list";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::ImageBuildOpts::builder(tmp.path().to_string_lossy())
        .labels([(label_key, value_a)])
        .tag(name_a)
        .build();
    let opts_b = opts::ImageBuildOpts::builder(tmp.path().to_string_lossy())
        .labels([(label_key, value_b)])
        .tag(name_b)
        .build();

    create_base_image(&podman, name_a, Some(opts_a.clone())).await;
    create_base_image(&podman, name_b, Some(opts_b.clone())).await;
    let image_a = images.get(name_a);
    let image_b = images.get(name_b);
    let full_id_a = get_image_full_id(&podman, name_a).await;
    let full_id_b = get_image_full_id(&podman, name_b).await;

    let filter = opts::ImageListFilter::LabelKey(label_key.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let mut list_result = images.list(&list_opts).await;
    if let Err(e) = list_result.as_ref() {
        if e.to_string().contains("does not exist in database") {
            // wait a bit in case a kill is executed at the same time
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            list_result = images.list(&list_opts).await;
        }
    }
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data
        .iter()
        .any(|data| data.id.as_ref().unwrap() == &full_id_a));
    assert!(list_data
        .iter()
        .any(|data| data.id.as_ref().unwrap() == &full_id_b));

    let filter = opts::ImageListFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    // This sometimes breaks when running all tests at the same time
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_a));
    assert!(!list_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_b));

    let filter = opts::ImageListFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(!list_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_a));
    assert!(list_data
        .iter()
        .any(|report| report.id.as_ref().unwrap() == &full_id_b));

    let _ = image_a.remove().await;
    let _ = image_b.remove().await;
}
