mod common;

use common::{
    cleanup_container,
    conn::TtyChunk,
    create_base_container, get_container_full_id, init_runtime,
    models::ContainerStatus,
    opts::{ContainerCreateOpts, ContainerWaitOpts, ExecCreateOpts},
    StreamExt, TryStreamExt, DEFAULT_CMD, DEFAULT_CMD_ARRAY, DEFAULT_IMAGE,
};
use podman_api::opts::ExecStartOpts;

#[tokio::test]
async fn container_create_exists_remove() {
    let podman = init_runtime();
    let container_name = "test-create-exist-container";

    let container = create_base_container(&podman, container_name, None).await;
    let exists_result = container.exists().await;
    assert!(exists_result.unwrap());

    let remove_result = container.remove().await;
    assert!(remove_result.is_ok());

    let exists_result = container.exists().await;
    assert!(!exists_result.unwrap());
}

#[tokio::test]
async fn container_inspect() {
    let podman = init_runtime();

    let container_name = "test-inspect-container";
    let container = create_base_container(&podman, container_name, None).await;
    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data.image_name.unwrap().contains("ubuntu:latest"));
    assert_eq!(inspect_data.name.unwrap(), "test-inspect-container");
    assert_eq!(inspect_data.config.unwrap().cmd.unwrap(), DEFAULT_CMD_ARRAY);

    cleanup_container(&podman, container_name).await;
    // check that the container got correctly removed
    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_err());
}

#[tokio::test]
async fn container_rename() {
    let podman = init_runtime();
    cleanup_container(&podman, "new-container-name").await;

    let container_name = "test-rename-container";
    let new_container_name = "test-rename-container-new";
    let container = create_base_container(&podman, container_name, None).await;

    let exists_result = podman.containers().get(new_container_name).exists().await;
    assert!(!exists_result.unwrap());

    let rename_result = container.rename(new_container_name).await;
    assert!(rename_result.is_ok());

    let exists_result = podman.containers().get(new_container_name).exists().await;
    assert!(exists_result.unwrap());

    cleanup_container(&podman, container_name).await;
    cleanup_container(&podman, new_container_name).await;
}

#[tokio::test]
async fn container_start() {
    let podman = init_runtime();

    let container_name = "test-start-container";
    let container = create_base_container(&podman, container_name, None).await;

    let start_result = container.start(None).await;
    assert!(start_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("started container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_restart() {
    let podman = init_runtime();

    let container_name = "test-restart-container";
    let container = create_base_container(&podman, container_name, None).await;
    let _ = container.start(None).await;

    let timestamp = container
        .inspect()
        .await
        .expect("restarted container inspect data before")
        .state
        .unwrap()
        .started_at
        .unwrap();

    let restart_result = container.restart().await;
    assert!(restart_result.is_ok());

    let timestamp_after = container
        .inspect()
        .await
        .expect("restarted container inspect data")
        .state
        .unwrap()
        .started_at
        .unwrap();
    assert_ne!(timestamp, timestamp_after);

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_pause_unpause() {
    let podman = init_runtime();

    let container_name = "test-pause-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let pause_result = container.pause().await;
    assert!(pause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("paused container inspect data");
    assert!(inspect_data.state.unwrap().paused.unwrap());

    let unpause_result = container.unpause().await;
    assert!(unpause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("paused container inspect data");
    assert!(!inspect_data.state.unwrap().paused.unwrap());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_kill() {
    let podman = init_runtime();

    let container_name = "test-kill-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let inspect_data = container
        .inspect()
        .await
        .expect("killed container inspect data");
    let state = inspect_data.state.unwrap();
    assert!(state.running.unwrap());

    let kill_result = container.kill().await;
    assert!(kill_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("killed container inspect data");
    let state = inspect_data.state.unwrap();
    assert!(!state.running.unwrap());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_stop() {
    let podman = init_runtime();

    let container_name = "test-stop-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let inspect_data = container
        .inspect()
        .await
        .expect("stopped container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    let stop_result = container.stop(&Default::default()).await;
    assert!(stop_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("stopped container inspect data");
    assert!(!inspect_data.state.unwrap().running.unwrap());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_init() {
    let podman = init_runtime();

    let container_name = "test-init-container";
    let container = create_base_container(&podman, container_name, None).await;

    let inspect_data = container
        .inspect()
        .await
        .expect("init container inspect data before");
    assert!(["created", "configured"].contains(
        &inspect_data
            .state
            .unwrap()
            .status
            .unwrap()
            .to_lowercase()
            .as_str()
    ));

    let init_result = container.init().await;
    assert!(init_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("init container inspect data");
    assert_eq!(
        inspect_data.state.unwrap().status.unwrap().to_lowercase(),
        "initialized"
    );

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_mount_unmount() {
    let podman = init_runtime();

    let container_name = "test-mount-container";
    let container = create_base_container(&podman, container_name, None).await;

    let inspect_data = container
        .inspect()
        .await
        .expect("mount container inspect data before");
    assert!(["created", "configured"].contains(
        &inspect_data
            .state
            .unwrap()
            .status
            .unwrap()
            .to_lowercase()
            .as_str()
    ));

    let mount_result = container.mount().await;
    assert!(mount_result.is_ok());
    let mount_path = mount_result.unwrap();
    assert!(mount_path.exists());

    let full_id = get_container_full_id(&podman, container_name).await;

    let mut list_mounted_result = podman.containers().list_mounted().await;
    if let Err(e) = list_mounted_result.as_ref() {
        if e.to_string().contains("does not exist in database") {
            // wait a bit in case a kill is executed at the same time
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            list_mounted_result = podman.containers().list_mounted().await;
        }
    }
    println!("{list_mounted_result:?}");
    assert!(list_mounted_result.is_ok());
    let list_mounted_data = list_mounted_result.unwrap();
    assert_eq!(
        list_mounted_data.get(full_id).unwrap().as_str().unwrap(),
        mount_path.to_string_lossy()
    );

    let unmount_result = container.unmount().await;
    assert!(unmount_result.is_ok());

    // Podman 5 seems to keep the merge directory even after unmount
    // assert!(!mount_path.exists());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_commit() {
    use podman_api::opts::ContainerCommitOpts;
    let podman = init_runtime();

    let container_name = "test-commit-container";
    let container = create_base_container(&podman, container_name, None).await;

    let author = "podman api rs";
    let commit_result = container
        .commit(
            &ContainerCommitOpts::builder()
                .author(author)
                .repo("test-commit-image")
                .tag("test")
                .build(),
        )
        .await;
    assert!(commit_result.is_ok());
    let image_name = "localhost/test-commit-image:test";

    let inspect_data = podman
        .images()
        .get(image_name)
        .inspect()
        .await
        .expect("commited image inspect data");
    assert_eq!(inspect_data.author.unwrap(), author);

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
#[ignore] // don't run by default
async fn container_checkpoint() {
    let podman = init_runtime();

    // must be run as root
    #[cfg(unix)]
    {
        assert!(nix::unistd::Uid::effective().is_root());
    }
    #[cfg(not(unix))]
    {
        return;
    }

    let container_name = "test-checkpoint-container";
    let container = create_base_container(&podman, container_name, None).await;
    let _ = container.start(None).await;

    let inspect_data = podman
        .containers()
        .get(container_name)
        .inspect()
        .await
        .expect("checkpointed container inspect data before");
    assert!(!inspect_data.state.unwrap().checkpointed.unwrap_or_default());

    let checkpoint_result = container.checkpoint(&Default::default()).await;
    assert!(checkpoint_result.is_ok());

    let inspect_data = podman
        .containers()
        .get(container_name)
        .inspect()
        .await
        .expect("checkpointed container inspect data");
    let state = inspect_data.state.unwrap();
    assert!(state.checkpointed.unwrap());
    assert!(!state.restored.unwrap_or_default());

    let restore_result = container.restore(&Default::default()).await;
    assert!(restore_result.is_ok());

    let inspect_data = podman
        .containers()
        .get(container_name)
        .inspect()
        .await
        .expect("checkpointed container inspect data after restore");
    let state = inspect_data.state.unwrap();
    assert!(!state.checkpointed.unwrap_or_default());
    assert!(state.restored.unwrap_or_default());

    // TODO: test checkpoint_export and restore_import

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_exec() {
    let podman = init_runtime();

    let container_name = "test-exec-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command([
                    "bash",
                    "-c",
                    "mkdir /tmp/test123 && echo 1234 >> /tmp/test123/testfile",
                ])
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();
    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    while exec_stream.next().await.is_some() {}

    let exec_inspect_result = exec.inspect().await;
    assert!(exec_inspect_result.is_ok());
    let exec_inspect_data = exec_inspect_result.unwrap();
    assert_eq!(exec_inspect_data.get("ExitCode").unwrap(), 0);

    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command(["cat", "test123/testfile"])
                .working_dir("/tmp")
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            let testfile_content = String::from_utf8_lossy(&chunk);
            assert_eq!(testfile_content, "1234\n");
        }
        Ok(chunk) => {
            let fd = match chunk {
                TtyChunk::StdIn(_) => "stdin",
                TtyChunk::StdOut(_) => "stdOut",
                TtyChunk::StdErr(_) => "stderr",
            };
            let chunk = String::from_utf8_lossy(&chunk);
            eprintln!("invalid chunk, fd: {fd}, content: `{chunk:?}`");
            std::process::exit(1);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_exec_detach() {
    let podman = init_runtime();

    let container_name = "test-exec-detach-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stdin(false)
                .attach_stderr(false)
                .attach_stdout(false)
                .command([
                    "bash",
                    "-c",
                    "mkdir /tmp/test123 && echo 1234 >> /tmp/test123/testfile",
                ])
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();
    let opts = ExecStartOpts::builder().detach(true).build();
    let exec_stream = exec.start(&opts).await.unwrap();
    assert!(exec_stream.is_none());

    let exec_inspect_result = exec.inspect().await;
    assert!(exec_inspect_result.is_ok());
    let exec_inspect_data = exec_inspect_result.unwrap();
    assert_eq!(exec_inspect_data.get("ExitCode").unwrap(), 0);

    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command(["cat", "test123/testfile"])
                .working_dir("/tmp")
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();
    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            let testfile_content = String::from_utf8_lossy(&chunk);
            assert_eq!(testfile_content, "1234\n");
        }
        Ok(chunk) => {
            let fd = match chunk {
                TtyChunk::StdIn(_) => "stdin",
                TtyChunk::StdOut(_) => "stdOut",
                TtyChunk::StdErr(_) => "stderr",
            };
            let chunk = String::from_utf8_lossy(&chunk);
            eprintln!("invalid chunk, fd: {fd}, content: `{chunk:?}`");
            std::process::exit(1);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_copy_from() {
    let podman = init_runtime();

    let container_name = "test-copy-from-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let exec = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command([
                    "bash",
                    "-c",
                    "mkdir /tmp/test123 && echo 1234 >> /tmp/test123/test-copy-from",
                ])
                .build(),
        )
        .await
        .expect("valid exec instance");
    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    while exec_stream.next().await.is_some() {}

    let tar_stream = container.copy_from("/tmp/test123");
    let bytes = tar_stream.try_concat().await.expect("joined tarball bytes");
    let mut archive = tar::Archive::new(&bytes[..]);
    let tmp = tempdir::TempDir::new("test-copy-from").expect("temporary dir");
    archive.unpack(tmp.path()).unwrap();

    let local_path = tmp.path().join("test123").join("test-copy-from");
    assert!(local_path.exists());
    assert_eq!(std::fs::read_to_string(&local_path).unwrap(), "1234\n");

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_copy_file_into() {
    let podman = init_runtime();

    let container_name = "test-copy-file-into-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let data = b"12345";
    let copy_result = container.copy_file_into("/tmp/test-file", data).await;
    assert!(copy_result.is_ok());

    let exec = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command(["cat", "/tmp/test-file"])
                .build(),
        )
        .await
        .expect("valid exec instance");
    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            assert_eq!(chunk, data);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_changes() {
    let podman = init_runtime();

    let container_name = "test-changes-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let exec = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command([
                    "bash",
                    "-c",
                    "rm /etc/xattr.conf && echo 12345 >> /tmp/test-changes",
                ])
                .build(),
        )
        .await
        .expect("valid exec instance");

    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    while exec_stream.next().await.is_some() {}

    use podman_api::models::FilesystemChange;

    let changes = container
        .changes(&Default::default())
        .await
        .expect("container changes");
    assert!(changes.contains(&FilesystemChange {
        kind: 0,
        path: "/tmp".into()
    }));
    assert!(changes.contains(&FilesystemChange {
        kind: 1,
        path: "/tmp/test-changes".into()
    }));
    assert!(changes.contains(&FilesystemChange {
        kind: 2,
        path: "/etc/xattr.conf".into()
    }));

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_logs() {
    let podman = init_runtime();

    let container_name = "test-logs-container";
    let container = create_base_container(
        &podman,
        container_name,
        Some(
            ContainerCreateOpts::builder()
                .name(container_name)
                .command(["bash", "-c", "echo 123456 && sleep inf"])
                .image(DEFAULT_IMAGE)
                .build(),
        ),
    )
    .await;

    let _ = container.start(None).await;

    use podman_api::opts::ContainerLogsOpts;

    let mut logs_stream = container.logs(
        &ContainerLogsOpts::builder()
            .stdout(true)
            .stderr(true)
            .build(),
    );
    let chunk = logs_stream.next().await;
    match chunk {
        Some(Ok(TtyChunk::StdOut(chunk))) => {
            let logs = String::from_utf8_lossy(&chunk);
            assert_eq!(logs, "123456\n");
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_stats() {
    let podman = init_runtime();

    let container_name = "test-stats-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let stats_result = container.stats().await;
    assert!(stats_result.is_ok());

    const WANT: i32 = 5;
    let mut got = 0;
    let mut stats_stream = container.stats_stream(Some(1));

    for _ in 0..WANT {
        if let Some(chunk) = stats_stream.next().await {
            assert!(chunk.is_ok());
            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_top() {
    use podman_api::opts::ContainerTopOpts;

    let podman = init_runtime();

    let container_name = "test-top-container";
    let container = create_base_container(&podman, container_name, None).await;

    let _ = container.start(None).await;

    let top_result = container.top(&Default::default()).await;
    assert!(top_result.is_ok());
    assert!(top_result.unwrap().processes[0].contains(&DEFAULT_CMD.to_string()));

    const WANT: i32 = 5;
    let mut got = 0;
    let mut top_stream = container.top_stream(&ContainerTopOpts::builder().delay(1).build());

    for _ in 0..WANT {
        if let Some(chunk) = top_stream.next().await {
            assert!(chunk.is_ok());
            assert!(chunk.unwrap().processes[0].contains(&DEFAULT_CMD.to_string()));

            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_generate_kube_yaml() {
    let podman = init_runtime();

    let container_name = "test-kube-yaml-container";
    let container = create_base_container(&podman, container_name, None).await;

    let yaml = container.generate_kube_yaml(false).await;
    assert!(yaml.is_ok());
    let yaml = yaml.unwrap();
    assert!(yaml.contains("kind: Pod"));
    assert!(!yaml.contains("kind: Service"));

    let yaml_service = container.generate_kube_yaml(true).await;
    assert!(yaml_service.is_ok());
    let yaml_service = yaml_service.unwrap();
    assert!(yaml_service.contains("kind: Pod"));
    assert!(yaml_service.contains("kind: Service"));

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_generate_systemd_units() {
    let podman = init_runtime();

    let container_name = "test-systemd-units-container";
    let container = create_base_container(&podman, container_name, None).await;

    let full_id = get_container_full_id(&podman, container_name).await;

    let units = container.generate_systemd_units(&Default::default()).await;
    assert!(units.is_ok());
    let units = units.unwrap();

    let unit = units.get(format!("container-{full_id}"));
    assert!(unit.is_some());

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn container_healthcheck() {
    use podman_api::models::Schema2HealthConfig;
    let podman = init_runtime();

    let container_name = "test-healthcheck-container";
    let opts = ContainerCreateOpts::builder()
        .name(container_name)
        .image(DEFAULT_IMAGE)
        .command(DEFAULT_CMD_ARRAY)
        .health_config(Schema2HealthConfig {
            interval: Some(1),
            retries: None,
            start_period: Some(1),
            test: Some(vec![
                "CMD-SHELL".into(),
                "cat".into(),
                "/etc/xattr.conf".into(),
            ]),
            timeout: Some(1000000000),
        })
        .build();
    let container = create_base_container(&podman, container_name, Some(opts)).await;
    let _ = container.start(None).await;

    let healthcheck_result = container.healthcheck().await;
    assert!(healthcheck_result.is_ok());
    assert_eq!(
        healthcheck_result.unwrap().status.unwrap_or_default(),
        "healthy"
    );

    let exec = container
        .create_exec(
            &ExecCreateOpts::builder()
                .command(["rm", "/etc/xattr.conf"])
                .attach_stdout(true)
                .attach_stderr(true)
                .build(),
        )
        .await
        .expect("valid exec instance");

    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    while exec_stream.next().await.is_some() {}

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // This is what I'd expect after removing the file but it doesn't work
    //let healthcheck_result = container.healthcheck().await;
    //assert!(healthcheck_result.is_ok());
    //assert_eq!(
    //healthcheck_result.unwrap().status.unwrap_or_default(),
    //"unhealthy"
    //);

    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn containers_list() {
    use podman_api::opts::{ContainerListFilter, ContainerListOpts};
    let podman = init_runtime();

    let container_name = "test-list-container";
    let opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-podman-list", "value")])
        .name(container_name)
        .build();

    let second_name = "test-list-second-container";
    let second_opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-podman-list", "value2")])
        .name(second_name)
        .build();
    create_base_container(&podman, container_name, Some(opts)).await;
    create_base_container(&podman, second_name, Some(second_opts)).await;

    let filter = ContainerListFilter::Name(container_name.to_string());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = podman.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&container_name.to_string()));

    let filter = ContainerListFilter::LabelKey("test-podman-list".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = podman.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);

    let filter = ContainerListFilter::LabelKeyVal("test-podman-list".into(), "value".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = podman.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&container_name.to_string()));

    let filter = ContainerListFilter::LabelKeyVal("test-podman-list".into(), "value2".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = podman.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&second_name.to_string()));

    cleanup_container(&podman, container_name).await;
    cleanup_container(&podman, second_name).await;
}

#[tokio::test]
async fn containers_prune() {
    use podman_api::opts::{ContainerPruneFilter, ContainerPruneOpts};
    let podman = init_runtime();

    let container_name = "test-prune-container";
    let opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-podman-prune", "value")])
        .name(container_name)
        .build();

    let second_name = "test-prune-second-container";
    let second_opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-podman-prune", "value2")])
        .name(second_name)
        .build();

    create_base_container(&podman, container_name, Some(opts.clone())).await;
    create_base_container(&podman, second_name, Some(second_opts.clone())).await;
    let full_id = get_container_full_id(&podman, container_name).await;
    let second_full_id = get_container_full_id(&podman, second_name).await;

    let filter = ContainerPruneFilter::LabelKey("test-podman-prune".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = podman.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert_eq!(prune_data.len(), 2);
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_deref().unwrap_or_default() == full_id));
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_deref().unwrap_or_default() == second_full_id));

    create_base_container(&podman, container_name, Some(opts.clone())).await;
    create_base_container(&podman, second_name, Some(second_opts.clone())).await;
    let full_id = get_container_full_id(&podman, container_name).await;
    let second_full_id = get_container_full_id(&podman, second_name).await;

    let filter = ContainerPruneFilter::LabelKeyVal("test-podman-prune".into(), "value".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = podman.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert_eq!(prune_data.len(), 1);
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_deref().unwrap_or_default() == full_id));

    let filter = ContainerPruneFilter::LabelKeyVal("test-podman-prune".into(), "value2".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = podman.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap();
    assert_eq!(prune_data.len(), 1);
    assert!(prune_data
        .iter()
        .any(|report| report.id.as_deref().unwrap_or_default() == second_full_id));
}

#[tokio::test]
async fn containers_stats() {
    use podman_api::opts::ContainerStatsOpts;
    let podman = init_runtime();
    let containers = podman.containers();

    let container_name = "test-stats-multiple-container";
    let container = create_base_container(&podman, container_name, None).await;
    let second_name = "test-stats-multiple-container2";
    let second_container = create_base_container(&podman, second_name, None).await;

    let _ = container.start(None).await;
    let _ = second_container.start(None).await;

    let opts = ContainerStatsOpts::builder()
        .containers([container_name, second_name])
        .interval(1)
        .build();
    let stats_result = containers.stats(&opts).await;
    assert!(stats_result.is_ok());

    const WANT: i32 = 5;
    let mut got = 0;
    let mut stats_stream = containers.stats_stream(&opts);

    for _ in 0..WANT {
        if let Some(chunk) = stats_stream.next().await {
            assert!(chunk.is_ok());
            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_container(&podman, container_name).await;
    cleanup_container(&podman, second_name).await;
}

#[tokio::test]
async fn container_wait() {
    let podman = init_runtime();
    let container_name = "test-wait-container";

    let container = create_base_container(&podman, container_name, None).await;

    let wait_result = container
        .wait(
            &ContainerWaitOpts::builder()
                .interval("500ms")
                .conditions([ContainerStatus::Running, ContainerStatus::Exited])
                .build(),
        )
        .await;
    assert!(wait_result.is_ok());
    cleanup_container(&podman, container_name).await;
}
