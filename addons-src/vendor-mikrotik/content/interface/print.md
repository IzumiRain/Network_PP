---
id = "cmd/mikrotik/interface-print"
kind = "command"
title = "/interface print"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "interfaces"]
risk = "info"
version = "RouterOS 7.x"
---
Lists all interfaces with type, link state and traffic flags.

## Command

```text
/interface print
/interface print stats
```

## Notes

- `R` marks a running interface (link up); no `R` means no cable/link or disabled port.
- `print stats` shows rx/tx byte and packet counters per interface.
- Ethernet-specific settings (speed, duplex, PoE) live under `/interface ethernet`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
