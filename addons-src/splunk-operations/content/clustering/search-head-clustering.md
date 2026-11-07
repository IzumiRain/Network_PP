---
id = "ref/splunk/search-head-clustering"
kind = "protocol"
title = "Search head clustering basics"
category = "reference/monitoring/splunk-ops/clustering"
tags = ["splunk", "clustering", "search-heads"]
version = "Splunk Enterprise 9.x"
---
A search head cluster keeps a set of search heads in sync (users, dashboards, scheduled searches) with an elected captain and a deployer.

## Notes

- Members replicate search artifacts (search jobs, alerts, knowledge objects); the captain coordinates the cluster.
- Apps and configuration are pushed to all members by the deployer - never edit member-local apps that the deployer manages.
- SHC gives availability and consistent scheduling; it is not a linear search-performance scaling mechanism.
- Scheduled searches are coordinated cluster-wide, preventing duplicate alert firing across members.
- Rolling upgrades and captain changes are routine maintenance topics; check captain and member status after each change.

Source: Splunk Enterprise Distributed Search — https://help.splunk.com
