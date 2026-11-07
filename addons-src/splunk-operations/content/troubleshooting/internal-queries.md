---
id = "cmd/splunk/internal-index-queries"
kind = "command"
title = "_internal troubleshooting queries"
category = "reference/monitoring/splunk-ops/troubleshooting"
tags = ["splunk", "spl", "troubleshooting", "internal"]
risk = "info"
version = "Splunk Enterprise 9.x"
---
Operational triage searches over the _internal index (splunkd.log and metrics.log).

## Command

```text
index=_internal log_level=ERROR source=*splunkd.log
index=_internal source=*metrics.log group=per_index_thruput series=web_data
index=_internal source=*metrics.log group=tcpin_connections
```

## Notes

- `group=per_index_thruput` shows ingest KB/s per index - the first check when "data stopped arriving".
- `group=tcpin_connections` lists connected forwarders; a missing host means the forwarder stopped or is blocked on 9997.
- Scope everything with `earliest=-15m` during live triage; _internal is very high volume.
- License usage lives in `index=_internal source=*license_usage.log` - same index, different source.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com
