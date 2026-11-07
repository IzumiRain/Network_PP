---
id = "ref/splunk/splunkd-log-components"
kind = "configuration"
title = "splunkd.log components and levels"
category = "reference/monitoring/splunk-ops/troubleshooting"
tags = ["splunk", "logging", "troubleshooting"]
version = "Splunk Enterprise 9.x"
---
splunkd.log is the main daemon log; its `component` and `log_level` fields make targeted triage searches possible.

## Example

```text
index=_internal source=*splunkd.log log_level=ERROR
index=_internal source=*splunkd.log component=HttpListener log_level=WARN*
```

## Notes

- Frequently useful components: IndexProcessor (parsing/indexing), HttpListener (REST/management), Metrics (internal metrics), KVStoreClient.
- Start triage at WARN/ERROR; drop to DEBUG only per-component and temporarily (`splunk set log-level <component> debug`) or volumes explode.
- Log rotation is size-based; search rather than reading files directly on disk when multiple instances are involved.
- Every Splunk process (splunkd, splunkweb, forwarders) reports into _internal, so one place covers the whole tier.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com
