use crate::docker_compose;
use redis::aio::{AsyncStream, Connection};
use redis::cluster::ClusterClient;
use redis::{AsyncCommands, Cmd, RedisConnectionInfo};
use serial_test::serial;
use std::pin::Pin;

pub async fn assert_ok(cmd: &mut Cmd, connection: &mut Connection) {
    assert_eq!(cmd.query_async(connection).await, Ok("OK".to_string()));
}

pub async fn assert_bytes(cmd: &mut Cmd, connection: &mut Connection, value: &[u8]) {
    assert_eq!(cmd.query_async(connection).await, Ok(value.to_vec()));
}

pub async fn redis_cluster_connection(address: &str) -> redis::cluster_async::ClusterConnection {
    let client = ClusterClient::new(vec![address]).unwrap();
    client.get_async_connection().await.unwrap()
}

pub async fn redis_single_connection(address: &str) -> redis::aio::Connection {
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let stream = Box::pin(stream) as Pin<Box<dyn AsyncStream + Send + Sync>>;

    redis::aio::Connection::new(&RedisConnectionInfo::default(), stream)
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn redis_cluster_1_1() {
    let _compose = docker_compose("../redis-cluster-1-1/docker-compose.yaml");

    let mut connection = redis_cluster_connection("redis://172.16.1.2:6380").await;

    let _: () = connection.set("foo", 42).await.unwrap();
    let value: i32 = connection.get("foo").await.unwrap();
    assert_eq!(value, 42);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn redis_cluster_1_many() {
    let _compose = docker_compose("../redis-cluster-1-many/docker-compose.yaml");

    let mut connection = redis_single_connection("172.16.1.9:6379").await;

    assert_ok(redis::cmd("SET").arg("foo").arg("value"), &mut connection).await;
    assert_bytes(
        redis::cmd("GET").arg("foo"),
        &mut connection,
        "value".as_bytes(),
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn redis_backup_cluster() {
    let _compose = docker_compose("../redis-backup-cluster/docker-compose.yaml");

    let mut connection = redis_single_connection("172.16.1.4:6379").await;

    assert_ok(redis::cmd("SET").arg("foo").arg("value"), &mut connection).await;
    assert_bytes(
        redis::cmd("GET").arg("foo"),
        &mut connection,
        "value".as_bytes(),
    )
    .await;
}
