---
id = "cmd/juniper/show-version"
kind = "command"
title = "show version"
category = "reference/vendors/juniper"
tags = ["juniper", "junos", "cli", "diagnostics"]
risk = "info"
version = "Junos OS 22.x"
---
Shows the Junos OS release, build, hostname, model and uptime.

## Command

```text
show version
show version detail
```

## Notes

- First check before opening a TAC case, upgrading, or matching release notes.
- `detail` also lists installed packages and jail versions.
- On mixed chassis (MX with SCB/fabric) it reports software per component.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
