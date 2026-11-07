---
id = "cmd/checkpoint/show-interface"
kind = "command"
title = "show interface (Gaia clish)"
category = "reference/firewalls/checkpoint"
tags = ["checkpoint", "gaia", "clish", "interfaces"]
risk = "info"
version = "Gaia R81.x"
---
Lists interfaces, or shows detail for one interface.

## Command

```text
show interface
show interface eth1
```

## Notes

- The list form shows every interface with its IPv4/IPv6 and on/off state.
- The name form shows address, link state, speed/duplex and MTU.
- Configuration happens in the same clish via `set interface ...` followed by `save config`.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
