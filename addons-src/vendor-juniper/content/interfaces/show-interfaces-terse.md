---
id = "cmd/juniper/show-interfaces-terse"
kind = "command"
title = "show interfaces terse"
category = "reference/vendors/juniper"
tags = ["juniper", "junos", "cli", "interfaces"]
risk = "info"
version = "Junos OS 22.x"
---
One-line status for every interface: link state, admin state and configured addresses.

## Command

```text
show interfaces terse
show interfaces ge-0/0/0 terse
```

## Notes

- Flags show the link (up/down) and admin state; protocol lines list IPv4/IPv6 addresses.
- Filter to one interface by name to speed up output on chassis with many ports.
- For errors, flapping and traffic counters use `show interfaces ge-0/0/0 extensive`.
- Works the same on EX, MX, SRX and QFX running Junos.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
