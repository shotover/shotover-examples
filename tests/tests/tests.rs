mod cassandra;
mod valkey;

use std::time::Duration;

use docker_compose_runner::{DockerCompose, Image};

fn docker_compose(yaml_path: &str) -> DockerCompose {
    DockerCompose::new(&IMAGE_WAITERS, |_| {}, yaml_path)
}

pub const IMAGE_WAITERS: [Image; 3] = [
    Image {
        name: "shotover/shotover-proxy:v0.6.0",
        log_regex_to_wait_for: r"accepting inbound connections",
        timeout: Duration::from_secs(120),
    },
    Image {
        name: "bitnami/cassandra:4.0.6",
        log_regex_to_wait_for: r"Startup complete",
        timeout: Duration::from_secs(240),
    },
    Image {
        name: "bitnami/valkey-cluster:7.2.5-debian-12-r4",
        //`Cluster state changed` is created by the node services
        //`Cluster correctly created` is created by the init service
        log_regex_to_wait_for: r"Cluster state changed|Cluster correctly created",
        timeout: Duration::from_secs(120),
    },
];
