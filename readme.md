# Shotover Examples

This repo provides example usages of shotover in various use cases.

## valkey-backup-cluster

This example demonstrates how to setup shotover to maintain a valkey backup cluster such that traffic could be diverted to the backup cluster in the case of complete failure of the primary cluster.
This is suitable for use cases where the data does not need to be 100% consistent between clusters but the service must not go down.

Shotover proxies messages to two distinct valkey clusters:

* primary cluster - shotover reads and writes to this cluster
* backup cluster - all writes are duplicated to the backup cluster
