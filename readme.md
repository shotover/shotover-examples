# Shotover Examples

The goal is to house example configs that run on the latest shotover release.

I'm just going to keep force pushing till I have something interesting here

## redis-backup-cluster

This example demonstrates how to setup shotover to maintain a redis backup cluster such that traffic could be diverted to the backup cluster in the case of complete failure of the primary cluster.
This is suitable for use cases where the data does not need to be 100% consistent between clusters but the service must not go down.

Shotover proxies messages to two distinct redis clusters:
* primary cluster - shotover reads and writes to this cluster
* backup cluster - all writes are duplicated to the backup cluster