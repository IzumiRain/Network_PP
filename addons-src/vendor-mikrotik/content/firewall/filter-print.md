---
id = "cmd/mikrotik/ip-firewall-filter-print"
kind = "command"
title = "/ip firewall filter print"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "firewall"]
risk = "info"
version = "RouterOS 7.x"
---
Lists packet-filter rules per chain (input, forward, output) with hit counters.

## Command

```text
/ip firewall filter print
/ip firewall filter print stats
/ip firewall filter print detail
```

## Notes

- `print stats` adds per-rule packet/byte counters - the first stop when traffic is unexpectedly blocked.
- `X` marks a disabled rule; `D` marks dynamic rules created by RouterOS services.
- Rules are matched top-down and the first match wins.
- `print detail` shows the full rule including comments and time-based options.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
