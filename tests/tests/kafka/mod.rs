use std::time::Duration;

use crate::docker_compose;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serial_test::serial;

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn kafka_1_many() {
    let _docker_compose = docker_compose("../kafka-1-many/docker-compose.yaml");

    let producer = producer_connection();

    let send = producer
        .send_result(FutureRecord::<(), _>::to("topic").payload("payload"))
        .unwrap();
    let delivery_status = tokio::time::timeout(Duration::from_secs(30), send)
        .await
        .expect("Timeout while receiving from producer")
        .unwrap()
        .unwrap();

    assert_eq!(delivery_status.offset, 0, "Unexpected offset");
}

fn producer_connection() -> FutureProducer {
    let mut client = ClientConfig::new();
    client
        .set("bootstrap.servers", "172.16.1.2")
        .set("broker.address.family", "v4");

    client
        .clone()
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap()
}
