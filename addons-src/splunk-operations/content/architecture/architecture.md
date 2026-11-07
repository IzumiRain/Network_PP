---
id = "ref/splunk/architecture"
kind = "protocol"
title = "Splunk architecture: forwarders, indexers, search head"
category = "reference/monitoring/splunk-ops/architecture"
tags = ["splunk", "architecture", "operations"]
version = "Splunk Enterprise 9.x"
---
Splunk separates collection (forwarders), storage/indexing (indexers) and searching (search heads), tied together by management and licensing services.

## Flow

```
Universal/Heavy Forwarder  --(9997)-->  Indexer(s)  <--(8089/search)--  Search Head
     collect + ship           store + index          search UI / apps
```

## Notes

- Universal forwarders ship raw/cooked data with minimal parsing; heavy forwarders can parse, filter and route (e.g. to HEC or third parties).
- Indexers store data in buckets per index and run the local part of distributed searches.
- The search head distributes searches to indexers and hosts apps, knowledge objects and users.
- Management traffic (deployment, licensing, clustering, REST) rides TCP 8089; data traffic uses 9997.
- The deployment server and license manager are roles that can live on a standalone instance or on the search head/manager.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
