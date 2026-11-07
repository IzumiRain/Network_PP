---
id = "cmd/asa/show-nat"
kind = "command"
title = "show nat"
category = "reference/vendors/cisco/asa/nat"
tags = ["asa", "cisco", "cli", "nat"]
risk = "info"
version = "ASA 9.16"
---
Displays the NAT policy table (Section 1/2/3 order) with per-rule translation and hit counters.

## Command

```cisco
show nat
show nat detail
```

## Notes

- Rules are evaluated top to bottom: manual NAT (section 1), object NAT (section 2), after-object NAT (section 3).
- Hit counts (translate/hit) reveal whether a rule actually matches; 0 hits on a path you expected is a clue.
- `show nat detail` appends the per-rule translation and creation flags (identity, net-to-net, unidirectional).
- First place to look when a connection is denied for no-match or unexpected translation in `packet-tracer`.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
