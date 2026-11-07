---
id = "cmd/mikrotik/system-resource-print"
kind = "command"
title = "/system resource print"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "system"]
risk = "info"
version = "RouterOS 7.x"
---
Shows router health and identity: uptime, CPU load, memory, version and board.

## Command

```text
/system resource print
/system resource print once
```

## Notes

- First check when a router feels slow: cpu-load, free-memory and cpu-count.
- The version line tells you which RouterOS release notes and upgrades apply.
- `print once` prints a single snapshot instead of a refreshing list.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
