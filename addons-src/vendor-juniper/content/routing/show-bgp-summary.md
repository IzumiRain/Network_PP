---
id = "cmd/juniper/show-bgp-summary"
kind = "command"
title = "show bgp summary"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "bgp", "routing"]
risk = "info"
version = "Junos OS 22.x"
---
Summarizes BGP sessions: peer counts per AS, session states and prefix counters.

## Command

```text
show bgp summary
show bgp summary instance <instance-name>
```

## Notes

- `EstablSz` is the number of established sessions; peers still in `Active`/`Connect` are not forming TCP.
- Per-peer columns show received/accepted/active/advertised prefix counts - useful to spot route churn.
- Per-neighbor detail and route flapping: `show bgp neighbor <address>`.
- Sessions in `Idle` often mean no route to the peer or a config/auth mismatch.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
