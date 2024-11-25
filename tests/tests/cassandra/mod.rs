use crate::docker_compose;
use futures::StreamExt;
use scylla::{Session, SessionBuilder};
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn cassandra_1_1() {
    let _docker_compose = docker_compose("../cassandra-1-1/docker-compose.yaml");

    let connection = cassandra_connection("172.16.1.2:9043").await;

    let iter = connection
        .query_iter("SELECT native_port FROM system.peers_v2;", &[])
        .await
        .unwrap()
        .rows_stream::<(i32,)>()
        .unwrap();

    let results: Vec<_> = iter.collect().await;
    let results: Vec<_> = results.into_iter().map(|x| x.unwrap().0).collect();

    assert_eq!(results, [9043, 9043])
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn cassandra_1_many() {
    let _docker_compose = docker_compose("../cassandra-1-many/docker-compose.yaml");

    let connection = cassandra_connection("172.16.1.5:9042").await;

    let iter = connection
        .query_iter("SELECT native_port FROM system.peers_v2;", &[])
        .await
        .unwrap()
        .rows_stream::<(i32,)>()
        .unwrap();
    let results: Vec<_> = iter.collect().await;
    let results: Vec<_> = results.into_iter().map(|x| x.unwrap().0).collect();

    assert_eq!(results, [])
}

async fn cassandra_connection(address: &str) -> Session {
    SessionBuilder::new()
        .known_nodes([address])
        .user("cassandra", "cassandra")
        .build()
        .await
        .unwrap()
}
