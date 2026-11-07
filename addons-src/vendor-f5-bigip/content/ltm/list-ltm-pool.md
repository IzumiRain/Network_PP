---
id = "cmd/f5-bigip/list-ltm-pool"
kind = "command"
title = "list ltm pool"
category = "reference/devices/f5-bigip/ltm"
tags = ["f5", "bigip", "tmsh", "ltm"]
risk = "info"
version = "BIG-IP 16.1"
---
Prints the pool configuration: members with their ports, load balancing method and attached health monitors.

## Command

```bash
tmsh list ltm pool
tmsh list ltm pool /Common/pool-app
```

## Notes

- Compare the configured members against reality: missing or wrong member IP/port is a classic 502 source.
- The load-balancing-method field (round-robin, least-connections-member) explains uneven distribution.
- Missing monitor entries means members are up regardless of real service health.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
