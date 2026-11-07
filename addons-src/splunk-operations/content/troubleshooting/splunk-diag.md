---
id = "cmd/splunk/diag"
kind = "command"
title = "splunk diag (support diagnostic bundle)"
category = "reference/monitoring/splunk-ops/troubleshooting"
tags = ["splunk", "cli", "troubleshooting", "diag"]
risk = "info"
version = "Splunk Enterprise 9.x"
---
Collects logs, configuration and environment info into a .diag archive for Splunk Support.

## Command

```bash
splunk diag
```

## Notes

- Output lands in `$SPLUNK_HOME/var/run/diag/` as a .diag file; credentials are scrubbed before upload.
- Diags can get large on busy systems - check size and, if needed, limit log inclusion before sending.
- Collect one diag per affected instance (forwarder, indexer, search head) when the problem spans tiers.
- Pair the diag with a reproduction description and time window; support uses the timestamps to focus on the right slice.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com
