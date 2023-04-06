mod cassandra;
mod redis;

use docker_compose_runner::{DockerCompose, Image};

fn docker_compose(yaml_path: &str) -> DockerCompose {
    DockerCompose::new(
        &[
            Image {
                name: "shotover/shotover-proxy:v0.1.10",
                log_regex_to_wait_for: r"accepting inbound connections",
            },
            Image {
                name: "bitnami/cassandra:4.0.6",
                log_regex_to_wait_for: r"Startup complete",
            },
            Image {
                name: "bitnami/redis-cluster:6.0-debian-10",
                //`Cluster state changed` is created by the node services
                //`Cluster correctly created` is created by the init service
                log_regex_to_wait_for: r"Cluster state changed|Cluster correctly created",
            },
        ],
        |_| {},
        yaml_path,
    )
}
