#![cfg(unix)] //temporary
mod common;

use common::{
    api::Secret, cleanup_container, conn, create_base_container, init_runtime, models, opts,
    Podman, StreamExt, DEFAULT_IMAGE,
};

//####################################################################################################

async fn create_base_secret(
    podman: &Podman,
    name: &str,
    value: &str,
    opts: Option<opts::SecretCreateOpts>,
) -> Secret {
    let secrets = podman.secrets();
    let _ = secrets.get(name).delete().await;

    let opts = opts.unwrap_or_else(|| opts::SecretCreateOpts::builder(name).build());

    secrets.create(&opts, value).await.expect("created secret")
}

//####################################################################################################

#[tokio::test]
async fn secret_create_remove() {
    let podman = init_runtime();

    let secret = create_base_secret(&podman, "test-create-secret", "test-value", None).await;
    assert!(secret.delete().await.is_ok());
}

#[tokio::test]
async fn secret_inspect() {
    let podman = init_runtime();

    let secret_name = "test-inspect-secret";
    let secret = create_base_secret(&podman, secret_name, "test-value", None).await;

    let inspect_result = secret.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    let spec = inspect_data.spec.unwrap();
    assert_eq!(spec.name.unwrap(), secret_name);
    assert_eq!(spec.driver.unwrap().name.unwrap(), "file");
    assert!(secret.delete().await.is_ok());
}

#[tokio::test]
async fn secret_create_read_value() {
    let podman = init_runtime();

    let secret_name = "test-create-read-secret";
    let secret_value = "test-value";
    let secret = create_base_secret(&podman, secret_name, secret_value, None).await;

    let secret_target: String = "/tmp/my-secret-value".into();
    let secret_def = models::Secret {
        gid: None,
        uid: None,
        mode: None,
        source: Some(secret_name.to_string()),
        target: Some(secret_target.clone()),
    };
    let container_name = "test-secret-read-container";
    let opts = opts::ContainerCreateOpts::builder()
        .name(container_name)
        .secrets([secret_def])
        .image(DEFAULT_IMAGE)
        .command(["cat", &secret_target])
        .build();
    let container = create_base_container(&podman, container_name, Some(opts)).await;
    container.start(None).await.expect("started container");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut logs_stream = container.logs(
        &opts::ContainerLogsOpts::builder()
            .stdout(true)
            .stderr(true)
            .build(),
    );
    let chunk = logs_stream.next().await;
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();
    assert!(chunk.is_ok());
    let chunk = chunk.unwrap();
    assert!(matches!(chunk, conn::TtyChunk::StdOut(_)));
    if let conn::TtyChunk::StdOut(data) = chunk {
        let s = String::from_utf8_lossy(&data);
        assert_eq!(s, format!("\"{secret_value}\""));
    } else {
        unreachable!();
    }
    assert!(secret.delete().await.is_ok());
    cleanup_container(&podman, container_name).await;
}

#[tokio::test]
async fn secret_list() {
    let podman = init_runtime();
    let secrets = podman.secrets();

    let name_a = "test-list-secret";
    let name_b = "test-list-secret2";
    let value_a = "value_a";
    let value_b = "value_b";

    let secret_a = create_base_secret(&podman, name_a, value_a, None).await;
    let secret_b = create_base_secret(&podman, name_b, value_b, None).await;

    let list_result = secrets.list().await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert!(!list_data.is_empty());
    assert!(list_data
        .iter()
        .any(|data| data.spec.as_ref().unwrap().name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|data| data.spec.as_ref().unwrap().name.as_ref().unwrap() == name_b));

    let _ = secret_a.delete().await;
    let _ = secret_b.delete().await;
}
