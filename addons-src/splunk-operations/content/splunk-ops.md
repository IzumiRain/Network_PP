---
id = "doc/monitoring/splunk-ops/splunk-ops"
kind = "configuration"
title = "Splunk Operations"
category = "reference/monitoring/splunk-ops"
tags = ["splunk", "architecture", "operations", "clustering", "indexers", "search-heads", "config", "inputs", "forwarding", "props", "transforms", "deployment", "forwarder-management", "upgrade", "indexes", "retention", "licensing", "kvstore", "cli", "monitoring", "troubleshooting", "btool", "spl", "internal", "diag", "logging", "model-doc"]
risk = "info"
version = "Splunk Enterprise 9.x"
---
The complete Splunk Operations document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Architecture
<!-- section-tags: splunk, architecture, operations -->


### Splunk architecture: forwarders, indexers, search head

Splunk separates collection (forwarders), storage/indexing (indexers) and searching (search heads), tied together by management and licensing services.

#### Flow

```
Universal/Heavy Forwarder  --(9997)-->  Indexer(s)  <--(8089/search)--  Search Head
     collect + ship           store + index          search UI / apps
```

#### Notes

- Universal forwarders ship raw/cooked data with minimal parsing; heavy forwarders can parse, filter and route (e.g. to HEC or third parties).
- Indexers store data in buckets per index and run the local part of distributed searches.
- The search head distributes searches to indexers and hosts apps, knowledge objects and users.
- Management traffic (deployment, licensing, clustering, REST) rides TCP 8089; data traffic uses 9997.
- The deployment server and license manager are roles that can live on a standalone instance or on the search head/manager.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


## Clustering
<!-- section-tags: splunk, clustering, indexers, search-heads -->


### Indexer clustering basics

Indexer clusters replicate buckets across peers so data survives peer loss; a manager (formerly master) coordinates.

#### Key terms

| Role | Purpose |
|---|---|
| Cluster manager (9.x) / master | Coordinates peers, replication, maintenance mode |
| Peer | Indexes data and serves replicated copies |
| Replication factor (RF) | Copies of each bucket that must exist |
| Search factor (SF) | Searchable copies retained (hot/warm only) |

#### Notes

- Under-replicated buckets mean the cluster is degraded; check replication health before maintenance or upgrades.
- Maintenance mode pauses bucket-fixup activities for planned work - remember to disable it afterwards.
- Forwarders load-balance across peers automatically; RF is handled by the cluster, not by outputs.conf.
- Common sizing: RF=3 with SF=2 for production clusters.

Source: Splunk Enterprise Managing Indexers and Clusters — https://help.splunk.com


### Search head clustering basics

A search head cluster keeps a set of search heads in sync (users, dashboards, scheduled searches) with an elected captain and a deployer.

#### Notes

- Members replicate search artifacts (search jobs, alerts, knowledge objects); the captain coordinates the cluster.
- Apps and configuration are pushed to all members by the deployer - never edit member-local apps that the deployer manages.
- SHC gives availability and consistent scheduling; it is not a linear search-performance scaling mechanism.
- Scheduled searches are coordinated cluster-wide, preventing duplicate alert firing across members.
- Rolling upgrades and captain changes are routine maintenance topics; check captain and member status after each change.

Source: Splunk Enterprise Distributed Search — https://help.splunk.com


## Config
<!-- section-tags: splunk, config, inputs, forwarding, props, transforms -->


### inputs.conf basics

inputs.conf defines what a Splunk instance collects: files, TCP/UDP ports, scripts and Windows sources.

#### Example

```ini
[monitor:///var/log/nginx/access.log]
index = web
sourcetype = nginx:access
disabled = 0

[monitor:///var/log/app/*.log]
index = app
sourcetype = app:log
```

#### Notes

- Stanzas live in `$SPLUNK_HOME/etc/system/local/inputs.conf` or app-scoped under `etc/apps/<app>/local/` - app-scoped is the deployable pattern.
- `index` must exist on the receiving indexers or data lands in `main`/fails; validate with `btool indexes list`.
- Set `sourcetype` explicitly at collection time - it anchors all later props.conf parsing.
- `disabled = 1` silences an input without deleting the stanza; useful during migrations.
- The CLI (`splunk add monitor /path`) is a convenient generator for the same stanzas.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### outputs.conf basics (forwarding)

outputs.conf controls where a forwarder sends data and how it load-balances across indexers.

#### Example

```ini
[tcpout]
defaultGroup = prod_indexers

[tcpout:prod_indexers]
server = idx1.example.com:9997, idx2.example.com:9997, idx3.example.com:9997
```

#### Notes

- 9997 is the standard receiving (Splunk-to-Splunk) port; verify `SPLUNK_TCP` receiving is enabled on the indexers.
- Forwarders auto-load-balance across the listed servers and fail over on connection loss.
- In an indexer cluster, forwarders list the peer indexers directly or use indexer discovery instead of static lists.
- `splunk btool outputs list --debug` shows the merged, effective forwarding configuration per instance.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### props.conf and transforms.conf basics

props.conf applies per-source/sourcetype settings (line breaking, timestamps); transforms.conf implements regex transformations it references.

#### Example

```ini
# props.conf
[nginx:access]
SHOULD_LINEMERGE = false
LINE_BREAKER = ([\r\n]+)
TIME_PREFIX = ^\[
TIME_FORMAT = %d/%b/%Y:%H:%M:%S %z
MAX_TIMESTAMP_LOOKAHEAD = 25
TRUNCATE = 10000
```

```ini
# transforms.conf (masking pattern)
[mask_account_number]
REGEX = (?m)^(.*)\d{12}(\d{4})(.*)$
FORMAT = $1XXXXXXXXXXXX$2$3
DEST_KEY = _raw
```

#### Notes

- Index-time props (LINE_BREAKER, TIME_*, TRUNCATE) affect data as it is parsed - test on a lab indexer before rolling out; bad settings cannot be un-applied to already-indexed data.
- transforms.conf entries are referenced from props.conf via REPORT-, TRANSFORMS- or SEDCMD-style keys for extractions, masks and routing.
- Keep both files together in one app so the deployable unit is self-contained.
- Search-time extractions (REPORT-/EXTRACT-) are safer than index-time fields; prefer them unless index-time is required.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


## Deployment
<!-- section-tags: splunk, deployment, forwarder-management, upgrade, operations -->


### Deployment server and serverclass.conf

The deployment server pushes apps to forwarders; serverclass.conf maps which apps go to which clients.

#### Example

```ini
# serverclass.conf
[serverClass:linux_forwarders]
whitelist.0 = srv-linux-*
[serverClass:linux_forwarders:app:base_inputs_linux]
```

#### Notes

- Apps to be distributed live in `$SPLUNK_HOME/etc/deployment-apps/`; serverclass targets them by whitelist/blacklist or machine types.
- Clients poll the server on a configured interval (phoneHomeIntervalInSecs on the forwarder side) and download app changes automatically.
- App state changes take effect on the client at next poll or restart, depending on app configuration.
- Push changes with `splunk reload deploy-server` after editing serverclass.conf; validate mappings with `splunk show deploy-server` style checks or the UI.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### deploymentclient.conf (forwarder side)

deploymentclient.conf points a forwarder at its deployment server for app management.

#### Example

```ini
[deploymentClient]
targetUri = deploy01.example.com:8089

[target-broker:deploymentClient]
targetUri = deploy01.example.com:8089
```

#### Notes

- Target port is the management port 8089, not the data port 9997.
- Once managed, apps the deployment server owns are reinstalled if removed locally - fix drift at the server, not on the forwarder.
- `phoneHomeIntervalInSecs` tunes polling frequency; lowering it helps during migrations, keep it moderate at scale.
- Verify management connectivity first: `telnet deploy01.example.com 8089` / `splunk btool deploymentclient list --debug`.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### Upgrade gotchas

Splunk upgrades are straightforward when prepared and painful when rushed; a few steps prevent most incidents.

#### Checklist

- [ ] Read the release notes: deprecated features and changed defaults for the exact version jump
- [ ] Back up `$SPLUNK_HOME/etc` (apps, local configs, users) before touching binaries
- [ ] Verify supported upgrade paths - major jumps can require interim versions, especially with KV Store migrations
- [ ] Upgrade in a planned order (management/cluster roles first, then peers/search heads, then forwarders)
- [ ] Validate post-upgrade with the Monitoring Console and `index=_internal log_level=ERROR`

#### Notes

- KV Store engine migrations run on first start after major upgrades - budget extra downtime and check `splunk show kvstore-status`.
- Hold deployment-server app pushes until the upgraded tier is confirmed healthy, so new apps do not mask upgrade symptoms.
- Test upgrades on a lab instance built from a copy of production `$SPLUNK_HOME/etc` first.

Source: Splunk Enterprise Installation Manual — https://help.splunk.com


## Indexes
<!-- section-tags: splunk, config, indexes, retention -->


### indexes.conf basics

indexes.conf defines indexes: storage paths, size caps and retention behavior.

#### Example

```ini
[web_data]
homePath = $SPLUNK_DB/web_data/db
coldPath = $SPLUNK_DB/web_data/colddb
thawedPath = $SPLUNK_DB/web_data/thaweddb
maxTotalDataSizeMB = 512000
```

#### Notes

- homePath, coldPath and thawedPath are mandatory for every index stanza - a missing path breaks index creation.
- Deploy indexes as an app to indexers (and cluster manager in a cluster); applying only on peers causes manager/peer drift.
- maxTotalDataSizeMB and frozenTimePeriodInSecs drive retention; without a frozenPath, archived buckets are deleted.
- `splunk btool indexes list --debug` shows the effective merged definition including defaults per index.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### Index retention: frozenTimePeriodInSecs and frozen data

Data ages out when it is older than frozenTimePeriodInSecs or when size caps are exceeded; frozen data is deleted unless an archive path is set.

#### Example

```ini
[web_data]
frozenTimePeriodInSecs = 7776000      # 90 days
maxTotalDataSizeMB = 512000
# coldToFrozenDir = /archive/web_data   # uncomment to archive instead of delete
```

#### Notes

- Default frozenTimePeriodInSecs is roughly 6 years (188697600); set it explicitly per index to a deliberate value.
- Retention is evaluated per bucket by its newest event, not per individual event - recent data shares the bucket age.
- With no frozen destination configured (default), buckets reaching the criteria are deleted from disk.
- Size-based retention (maxTotalDataSizeMB / maxDataSize) can remove data earlier than the time policy when ingest is heavy.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


## Licensing
<!-- section-tags: splunk, licensing, operations -->


### License violations and warnings

Exceeding a daily license quota logs warnings; enough warnings within a window disables search until resolved.

#### Example

```text
index=_internal source=*license_usage.log type=Usage
```

#### Notes

- Data exceeding the quota is still indexed, but the event triggers a license warning - ingestion does not stop.
- More than 5 warnings in a rolling 30-day window disables search until the window clears or licensing is fixed.
- Check the daily usage query (`type=Usage`) to find which index/host drove the overage before deciding on a pool change.
- Temporary licenses and emergency/capacity changes behave differently - always review the current license terms before relying on behavior.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


## Monitoring
<!-- section-tags: splunk, kvstore, cli, monitoring, operations -->


### splunk show kvstore-status

Reports KV Store (MongoDB-backed) health, state and storage statistics.

#### Command

```bash
splunk show kvstore-status
```

#### Notes

- The KV Store backs app-managed collections (lookup tables, app state); Enterprise Security assets/notables rely on it heavily.
- Watch `state` (starting/ready) and storage growth; a stuck non-ready state blocks apps that depend on collections.
- Major version upgrades migrate the KV Store engine - check status right after the first start post-upgrade.
- On search heads with SHC, KV Store data is managed per the cluster; investigate the captain first when status diverges.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


### Monitoring Console (DMC)

The Splunk Monitoring Console is the built-in health dashboard for instance performance, queues and forwarder status.

#### Notes

- A standalone instance monitors itself by default; distributed deployments must switch the console to distributed mode and add instances.
- Monitoring targets include search activity, indexing performance, queue status (index queue, parsing) and forwarder connectivity.
- The console reads metrics via REST and _internal data, so the instance running it needs management access (8089) to every monitored node.
- First checks for "Splunk is slow" complaints: Monitoring Console resource usage and index/queue dashboards before diving into splunkd.log.
- Enable the forwarder monitoring dashboard to spot forwarders that stopped phoning home.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com


## Troubleshooting
<!-- section-tags: splunk, cli, troubleshooting, btool, spl, internal, diag, logging -->


### splunk btool (effective config check)

btool merges all conf layers and shows the effective configuration with the winning file per setting.

#### Command

```bash
splunk btool inputs list --debug
splunk btool props list nginx:access --debug
splunk btool indexes list web_data --debug
```

#### Notes

- `--debug` prints the file path of each line - the fastest way to learn which app wins a setting collision.
- btool reads conf files the way Splunk will; it does not restart services and changes nothing (read-only).
- Precedence runs default -> app/default -> app/local -> system/local within the search path; btool reflects it.
- If a setting seems ignored, first confirm it appears in btool output at all - misspelled keys or wrong stanza names are the usual cause.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com


### _internal troubleshooting queries

Operational triage searches over the _internal index (splunkd.log and metrics.log).

#### Command

```text
index=_internal log_level=ERROR source=*splunkd.log
index=_internal source=*metrics.log group=per_index_thruput series=web_data
index=_internal source=*metrics.log group=tcpin_connections
```

#### Notes

- `group=per_index_thruput` shows ingest KB/s per index - the first check when "data stopped arriving".
- `group=tcpin_connections` lists connected forwarders; a missing host means the forwarder stopped or is blocked on 9997.
- Scope everything with `earliest=-15m` during live triage; _internal is very high volume.
- License usage lives in `index=_internal source=*license_usage.log` - same index, different source.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com


### splunk diag (support diagnostic bundle)

Collects logs, configuration and environment info into a .diag archive for Splunk Support.

#### Command

```bash
splunk diag
```

#### Notes

- Output lands in `$SPLUNK_HOME/var/run/diag/` as a .diag file; credentials are scrubbed before upload.
- Diags can get large on busy systems - check size and, if needed, limit log inclusion before sending.
- Collect one diag per affected instance (forwarder, indexer, search head) when the problem spans tiers.
- Pair the diag with a reproduction description and time window; support uses the timestamps to focus on the right slice.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com


### splunkd.log components and levels

splunkd.log is the main daemon log; its `component` and `log_level` fields make targeted triage searches possible.

#### Example

```text
index=_internal source=*splunkd.log log_level=ERROR
index=_internal source=*splunkd.log component=HttpListener log_level=WARN*
```

#### Notes

- Frequently useful components: IndexProcessor (parsing/indexing), HttpListener (REST/management), Metrics (internal metrics), KVStoreClient.
- Start triage at WARN/ERROR; drop to DEBUG only per-component and temporarily (`splunk set log-level <component> debug`) or volumes explode.
- Log rotation is size-based; search rather than reading files directly on disk when multiple instances are involved.
- Every Splunk process (splunkd, splunkweb, forwarders) reports into _internal, so one place covers the whole tier.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com

