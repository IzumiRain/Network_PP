---
id = "cmd/asa/show-failover"
kind = "command"
title = "show failover"
category = "reference/vendors/cisco/asa/failover"
tags = ["asa", "cisco", "cli", "failover", "ha"]
risk = "info"
version = "ASA 9.16"
---
Failover pair status: active/standby roles, monitored interfaces and state links.

## Command

```cisco
show failover
show failover history
show monitor-interface
```

## Notes

- First line shows this unit's role (Active/Standby Ready) and the peer's; `Other host: Primary/Secondary` follows.
- Interfaces must list as Monitored and Normal; a Waiting/Failed monitored interface drives failover decisions.
- `show failover history` lists state transitions with reasons - the fastest route to why the pair flipped.
- Failover link (stateful or not) and the LAN-based failover interface health are shown at the end of the output.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
