---
id = "cmd/juniper/show-ospf-neighbor"
kind = "command"
title = "show ospf neighbor"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "ospf", "routing"]
risk = "info"
version = "Junos OS 22.x"
---
Lists OSPF adjacencies with interface, area, neighbor address and adjacency state.

## Command

```text
show ospf neighbor
show ospf neighbor extensive
```

## Notes

- Expect `Full` on established adjacencies; anything else deserves a look.
- Stuck in `ExStart`/`Exchange` usually means an MTU mismatch; `Init` points at hello/auth or access-list issues.
- For OSPFv3 use the equivalent `show ospf3 neighbor`.
- `extensive` adds timers, DR/BDR role and retransmission counters.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
