#![cfg(unix)] //temporary
mod common;

use common::{api::Pod, init_runtime, opts::PodCreateOpts, Podman, StreamExt, DEFAULT_IMAGE};

const DEFAULT_CMD: &str = "sleep inf";
const DEFAULT_CMD_ARRAY: &[&str] = &["sleep", "inf"];

//####################################################################################################

async fn cleanup_pod(podman: &Podman, name: &str) {
    let _ = podman.pods().get(name).remove().await;
}

async fn create_base_pod(podman: &Podman, name: &str, opts: Option<PodCreateOpts>) -> Pod {
    cleanup_pod(podman, name).await;

    let opts = opts.unwrap_or_else(|| {
        PodCreateOpts::builder()
            .name(name)
            .infra_command(DEFAULT_CMD_ARRAY)
            .infra_image(DEFAULT_IMAGE)
            .infra_name(format!("infra-{name}"))
            .build()
    });
    podman.pods().create(&opts).await.expect("created base pod");
    podman.pods().get(name)
}

async fn get_pod_full_id(podman: &Podman, name: &str) -> String {
    podman
        .pods()
        .get(name)
        .inspect()
        .await
        .map(|data| data.id)
        .expect("pod inspect data")
        .expect("pod full id")
}

async fn get_pod_container_id(pod: &Pod) -> String {
    pod.inspect()
        .await
        .expect("pod inspect data")
        .containers
        .expect("pod containers list")[0]
        .id
        .as_ref()
        .expect("container id")
        .clone()
}

//####################################################################################################

#[tokio::test]
async fn pod_create_exists_remove() {
    let podman = init_runtime();
    let pod_name = "test-create-exist-pod";

    cleanup_pod(&podman, pod_name).await;
    let exists_result = podman.pods().get(pod_name).exists().await;
    assert!(!exists_result.unwrap());

    let pod = create_base_pod(&podman, pod_name, None).await;

    let exists_result = pod.exists().await;
    assert!(exists_result.unwrap());

    let remove_result = pod.remove().await;
    assert!(remove_result.is_ok());

    let exists_result = pod.exists().await;
    assert!(!exists_result.unwrap());
}

#[tokio::test]
async fn pod_inspect() {
    let podman = init_runtime();

    let pod_name = "test-inspect-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;
    let inspect_result = pod.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data.containers.unwrap()[0]
        .name
        .as_ref()
        .unwrap()
        .contains(&format!("infra-{pod_name}")));
    assert_eq!(inspect_data.name.unwrap(), "test-inspect-pod");

    cleanup_pod(&podman, pod_name).await;
    // check that the pod got correctly removed
    let inspect_result = pod.inspect().await;
    assert!(inspect_result.is_err());
}

#[tokio::test]
async fn pod_start() {
    let podman = init_runtime();

    let pod_name = "test-start-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let start_result = pod.start().await;
    assert!(start_result.is_ok());

    let container_id = get_pod_container_id(&pod).await;
    let inspect_data = podman
        .containers()
        .get(container_id)
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_restart() {
    let podman = init_runtime();

    let pod_name = "test-restart-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;
    let _ = pod.start().await;

    let container_id = get_pod_container_id(&pod).await;
    let container = podman.containers().get(container_id);

    let timestamp = container
        .inspect()
        .await
        .expect("restarted pod inspect data before")
        .state
        .unwrap()
        .started_at
        .unwrap();

    let restart_result = pod.restart().await;
    assert!(restart_result.is_ok());

    let timestamp_after = container
        .inspect()
        .await
        .expect("restarted pod inspect data before")
        .state
        .unwrap()
        .started_at
        .unwrap();
    assert_ne!(timestamp, timestamp_after);

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_pause_unpause() {
    let podman = init_runtime();

    let pod_name = "test-pause-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let _ = pod.start().await;

    let container_id = get_pod_container_id(&pod).await;
    let container = podman.containers().get(container_id);

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(!inspect_data.state.unwrap().paused.unwrap());

    let pause_result = pod.pause().await;
    assert!(pause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(inspect_data.state.unwrap().paused.unwrap());

    let unpause_result = pod.unpause().await;
    assert!(unpause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(!inspect_data.state.unwrap().paused.unwrap());

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_kill() {
    let podman = init_runtime();

    let pod_name = "test-kill-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let _ = pod.start().await;
    let container_id = get_pod_container_id(&pod).await;
    let container = podman.containers().get(container_id);

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    let kill_result = pod.kill().await;
    assert!(kill_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(!inspect_data.state.unwrap().running.unwrap());

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_stop() {
    let podman = init_runtime();

    let pod_name = "test-stop-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let _ = pod.start().await;
    let container_id = get_pod_container_id(&pod).await;
    let container = podman.containers().get(container_id);

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    let stop_result = pod.stop().await;
    assert!(stop_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("pod container inspect data");
    assert!(!inspect_data.state.unwrap().running.unwrap());

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_top() {
    use podman_api::opts::PodTopOpts;

    let podman = init_runtime();

    let pod_name = "test-top-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let _ = pod.start().await;

    let top_result = pod.top(&Default::default()).await;
    assert!(top_result.is_ok());
    assert!(top_result.unwrap().processes[0].contains(&DEFAULT_CMD.to_string()));

    const WANT: i32 = 5;
    let mut got = 0;
    let mut top_stream = pod.top_stream(&PodTopOpts::builder().delay(1).build());

    for _ in 0..WANT {
        if let Some(chunk) = top_stream.next().await {
            assert!(chunk.is_ok());
            assert!(chunk.unwrap().processes[0].contains(&DEFAULT_CMD.to_string()));

            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pod_generate_systemd_units() {
    let podman = init_runtime();

    let pod_name = "test-systemd-units-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;

    let full_id = get_pod_full_id(&podman, pod_name).await;

    let units = pod.generate_systemd_units(&Default::default()).await;
    assert!(units.is_ok());
    let units = units.unwrap();

    let unit = units.get(format!("pod-{full_id}"));
    assert!(unit.is_some());

    cleanup_pod(&podman, pod_name).await;
}

#[tokio::test]
async fn pods_list() {
    use podman_api::opts::{PodListFilter, PodListOpts};
    let podman = init_runtime();

    let name_a = "test-list-pod";
    let opts_a = PodCreateOpts::builder()
        .infra_command(DEFAULT_CMD_ARRAY)
        .infra_image(DEFAULT_IMAGE)
        .labels([("test-podman-list", "value")])
        .name(name_a)
        .build();

    let name_b = "test-list-second-pod";
    let opts_b = PodCreateOpts::builder()
        .infra_command(DEFAULT_CMD_ARRAY)
        .infra_image(DEFAULT_IMAGE)
        .labels([("test-podman-list", "value2")])
        .name(name_b)
        .build();
    create_base_pod(&podman, name_a, Some(opts_a)).await;
    create_base_pod(&podman, name_b, Some(opts_b)).await;

    let filter = PodListFilter::Name(name_a.to_string());
    let opts = PodListOpts::builder().filter([filter]).build();
    let list_result = podman.pods().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert_eq!(list_data[0].name.as_ref().unwrap(), &name_a);

    let filter = PodListFilter::LabelKey("test-podman-list".into());
    let opts = PodListOpts::builder().filter([filter]).build();
    let list_result = podman.pods().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data
        .iter()
        .any(|it| it.name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|it| it.name.as_ref().unwrap() == name_b));

    let filter = PodListFilter::LabelKeyVal("test-podman-list".into(), "value".into());
    let opts = PodListOpts::builder().filter([filter]).build();
    let list_result = podman.pods().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert_eq!(list_data[0].name.as_ref().unwrap(), &name_a);

    let filter = PodListFilter::LabelKeyVal("test-podman-list".into(), "value2".into());
    let opts = PodListOpts::builder().filter([filter]).build();
    let list_result = podman.pods().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert_eq!(list_data[0].name.as_ref().unwrap(), &name_b);

    cleanup_pod(&podman, name_a).await;
    cleanup_pod(&podman, name_b).await;
}

#[tokio::test]
async fn pods_stats() {
    use podman_api::opts::PodStatsOpts;
    let podman = init_runtime();
    let pods = podman.pods();

    let pod_name = "test-stats-multiple-pod";
    let pod = create_base_pod(&podman, pod_name, None).await;
    let second_name = "test-stats-multiple-pod2";
    let second_pod = create_base_pod(&podman, second_name, None).await;

    let _ = pod.start().await;
    let _ = second_pod.start().await;

    let opts = PodStatsOpts::builder()
        .names_or_ids([pod_name, second_name])
        .build();

    const WANT: i32 = 5;
    let mut got = 0;
    let mut stats_stream = pods.stats(&opts);

    for _ in 0..WANT {
        if let Some(chunk) = stats_stream.next().await {
            assert!(chunk.is_ok());
            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_pod(&podman, pod_name).await;
    cleanup_pod(&podman, second_name).await;
}
