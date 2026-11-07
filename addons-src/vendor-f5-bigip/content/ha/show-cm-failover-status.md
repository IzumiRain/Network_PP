---
id = "cmd/f5-bigip/show-cm-failover-status"
kind = "command"
title = "show cm failover-status"
category = "reference/devices/f5-bigip/ha"
tags = ["f5", "bigip", "tmsh", "ha"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows the HA failover state of the unit: active or standby, plus the failover color and reason.

## Command

```bash
tmsh show cm failover-status
```

## Notes

- Always confirm which unit is active before maintenance; run this on both peers.
- Color green with a stable state is healthy; amber/red states indicate network or peer problems.
- Pair with show cm sync-status to confirm both that the unit is active and the config is synchronized.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
