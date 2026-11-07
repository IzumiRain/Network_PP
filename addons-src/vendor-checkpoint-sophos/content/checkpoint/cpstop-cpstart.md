---
id = "cmd/checkpoint/cpstop-cpstart"
kind = "command"
title = "cpstop / cpstart"
category = "reference/vendors/check-point"
tags = ["checkpoint", "cli", "services"]
risk = "high"
version = "Gaia R81.x"
---
Stops or starts all Check Point security product daemons on the machine.

## Command

```text
cpstop
cpstart
```

## Notes

- On a standalone gateway without HA, cpstop halts traffic handling immediately.
- In a cluster, stopping one member triggers failover - still announce the maintenance.
- Bring everything back with cpstart and verify with cpview or `cpstat os -f all`.
- Run from Expert mode; never leave a gateway stopped after a troubleshooting session.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
