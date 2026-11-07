---
id = "cmd/splunk/show-kvstore-status"
kind = "command"
title = "splunk show kvstore-status"
category = "reference/monitoring/splunk-ops/monitoring"
tags = ["splunk", "kvstore", "cli"]
risk = "info"
version = "Splunk Enterprise 9.x"
---
Reports KV Store (MongoDB-backed) health, state and storage statistics.

## Command

```bash
splunk show kvstore-status
```

## Notes

- The KV Store backs app-managed collections (lookup tables, app state); Enterprise Security assets/notables rely on it heavily.
- Watch `state` (starting/ready) and storage growth; a stuck non-ready state blocks apps that depend on collections.
- Major version upgrades migrate the KV Store engine - check status right after the first start post-upgrade.
- On search heads with SHC, KV Store data is managed per the cluster; investigate the captain first when status diverges.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
