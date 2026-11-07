---
id = "ref/splunk/indexer-clustering"
kind = "protocol"
title = "Indexer clustering basics"
category = "reference/monitoring/splunk-ops/clustering"
tags = ["splunk", "clustering", "indexers"]
version = "Splunk Enterprise 9.x"
---
Indexer clusters replicate buckets across peers so data survives peer loss; a manager (formerly master) coordinates.

## Key terms

| Role | Purpose |
|---|---|
| Cluster manager (9.x) / master | Coordinates peers, replication, maintenance mode |
| Peer | Indexes data and serves replicated copies |
| Replication factor (RF) | Copies of each bucket that must exist |
| Search factor (SF) | Searchable copies retained (hot/warm only) |

## Notes

- Under-replicated buckets mean the cluster is degraded; check replication health before maintenance or upgrades.
- Maintenance mode pauses bucket-fixup activities for planned work - remember to disable it afterwards.
- Forwarders load-balance across peers automatically; RF is handled by the cluster, not by outputs.conf.
- Common sizing: RF=3 with SF=2 for production clusters.

Source: Splunk Enterprise Managing Indexers and Clusters — https://help.splunk.com
