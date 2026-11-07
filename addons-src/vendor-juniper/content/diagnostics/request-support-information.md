---
id = "cmd/juniper/request-support-information"
kind = "command"
title = "request support information"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "diagnostics", "support"]
risk = "info"
version = "Junos OS 22.x"
---
Collects a full technical-support dump: interfaces, protocols, chassis state and logs.

## Command

```text
request support information
request support information | save /var/tmp/support.txt
```

## Notes

- The output JTAC expects at the start of a case - run it early during an incident.
- Output is very long; redirect with `| save` to a file and pull it via SCP.
- Capture before restarting anything, so the broken state is preserved.
- Read-only: safe to run on a production device.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
