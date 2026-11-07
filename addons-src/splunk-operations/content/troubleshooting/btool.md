---
id = "cmd/splunk/btool"
kind = "command"
title = "splunk btool (effective config check)"
category = "reference/monitoring/splunk-ops/troubleshooting"
tags = ["splunk", "cli", "troubleshooting", "btool"]
risk = "info"
version = "Splunk Enterprise 9.x"
---
btool merges all conf layers and shows the effective configuration with the winning file per setting.

## Command

```bash
splunk btool inputs list --debug
splunk btool props list nginx:access --debug
splunk btool indexes list web_data --debug
```

## Notes

- `--debug` prints the file path of each line - the fastest way to learn which app wins a setting collision.
- btool reads conf files the way Splunk will; it does not restart services and changes nothing (read-only).
- Precedence runs default -> app/default -> app/local -> system/local within the search path; btool reflects it.
- If a setting seems ignored, first confirm it appears in btool output at all - misspelled keys or wrong stanza names are the usual cause.

Source: Splunk Enterprise Troubleshooting Manual — https://help.splunk.com
