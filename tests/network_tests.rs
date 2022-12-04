#![cfg(unix)] //temporary
mod common;

use common::{create_base_container, create_base_network, init_runtime, opts};

#[tokio::test]
async fn network_create_exists_remove() {
    let podman = init_runtime();

    let network = create_base_network(&podman, "test-create-network", None).await;

    assert!(network.exists().await.unwrap());
    assert!(network.remove().await.is_ok());
    assert!(!network.exists().await.unwrap());
    let network = create_base_network(&podman, "test-create-network", None).await;
    assert!(network.exists().await.unwrap());
    assert!(network.delete().await.is_ok());
    assert!(!network.exists().await.unwrap());
}

#[tokio::test]
async fn network_inspect() {
    let podman = init_runtime();
    let networks = podman.networks();

    let network_name = "test-inspect-network";
    create_base_network(&podman, network_name, None).await;

    let network = networks.get(network_name);

    let inspect_result = network.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data.name.as_ref().unwrap().contains(network_name));
    assert!(network.remove().await.is_ok());
}

#[tokio::test]
async fn network_prune() {
    let podman = init_runtime();
    let networks = podman.networks();

    let name_a = "test-prune-network";
    let name_b = "test-prune-network2";

    let label_key = "test-prune";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::NetworkCreateOpts::builder()
        .labels([(label_key, value_a)])
        .name(name_a)
        .build();
    let opts_b = opts::NetworkCreateOpts::builder()
        .labels([(label_key, value_b)])
        .name(name_b)
        .build();

    create_base_network(&podman, name_a, Some(opts_a.clone())).await;
    create_base_network(&podman, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);
    assert!(network_a.exists().await.unwrap());
    assert!(network_b.exists().await.unwrap());

    let filter = opts::NetworkPruneFilter::LabelKey(label_key.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.name.as_ref().unwrap() == name_a));
    assert!(prune_data
        .iter()
        .any(|report| report.name.as_ref().unwrap() == name_b));
    assert!(!network_a.exists().await.unwrap());
    assert!(!network_b.exists().await.unwrap());

    create_base_network(&podman, name_a, Some(opts_a.clone())).await;
    create_base_network(&podman, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);
    assert!(network_a.exists().await.unwrap());
    assert!(network_b.exists().await.unwrap());

    let filter = opts::NetworkPruneFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.name.as_ref().unwrap() == name_a));
    assert!(!prune_data
        .iter()
        .any(|report| report.name.as_ref().unwrap() == name_b));
    assert!(!network_a.exists().await.unwrap());
    assert!(network_b.exists().await.unwrap());

    let filter = opts::NetworkPruneFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert!(prune_data
        .iter()
        .any(|report| report.name.as_ref().unwrap() == name_b));

    assert!(!network_a.exists().await.unwrap());
    assert!(!network_b.exists().await.unwrap());
}

#[tokio::test]
async fn network_list() {
    let podman = init_runtime();
    let networks = podman.networks();

    let name_a = "test-list-network";
    let name_b = "test-list-network2";

    let label_key = "test-list";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::NetworkCreateOpts::builder()
        .labels([(label_key, value_a)])
        .name(name_a)
        .build();
    let opts_b = opts::NetworkCreateOpts::builder()
        .labels([(label_key, value_b)])
        .name(name_b)
        .build();

    create_base_network(&podman, name_a, Some(opts_a.clone())).await;
    create_base_network(&podman, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);

    let filter = opts::NetworkListFilter::LabelKey(label_key.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let filter = opts::NetworkListFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    // This sometimes breaks when running all tests at the same time
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(!list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let filter = opts::NetworkListFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(!list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let _ = network_a.remove().await;
    let _ = network_b.remove().await;
}

#[tokio::test]
#[ignore] // requires root access
async fn network_connect_disconnect() {
    let podman = init_runtime();

    let network_name = "test-connect-network";
    let container_name = "test-connect-network-container";
    let network = create_base_network(&podman, network_name, None).await;

    let container = create_base_container(&podman, container_name, None).await;

    let opts = opts::NetworkConnectOpts::builder()
        .container(container_name)
        .build();

    let connect_result = network.connect_container(&opts).await;
    assert!(connect_result.is_ok());
    connect_result.unwrap();

    //let container_data = container.inspect().await;
    //log::error!("{container_data:#?}");

    let _ = network.remove().await;
    let _ = container.remove().await;
}
