use crate::docker_compose;
use scylla::frame::response::result::{CqlValue, Row};
use scylla::{Session, SessionBuilder};
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn cassandra_1_1() {
    let _docker_compose = docker_compose("../cassandra-1-1/docker-compose.yaml");

    let connection = cassandra_connection("172.16.1.2:9043").await;
    let results = connection
        .query("SELECT native_port FROM system.peers_v2;", &[])
        .await
        .unwrap();

    assert_eq!(
        results.rows.unwrap(),
        &[
            Row {
                columns: vec!(Some(CqlValue::Int(9043)))
            },
            Row {
                columns: vec!(Some(CqlValue::Int(9043)))
            }
        ]
    )
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn cassandra_1_many() {
    let _docker_compose = docker_compose("../cassandra-1-many/docker-compose.yaml");

    let connection = cassandra_connection("172.16.1.5:9042").await;

    let results = connection
        .query("SELECT native_port FROM system.peers_v2;", &[])
        .await
        .unwrap();
    assert_eq!(results.rows.unwrap(), &[])
}

async fn cassandra_connection(address: &str) -> Session {
    SessionBuilder::new()
        .known_nodes([address])
        .user("cassandra", "cassandra")
        .build()
        .await
        .unwrap()
}
