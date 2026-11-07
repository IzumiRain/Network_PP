---
id = "cmd/fortiweb/show-server-pool"
kind = "command"
title = "show server-pool"
category = "reference/firewalls/fortiweb/policies"
tags = ["fortiweb", "waf", "cli", "policies"]
risk = "info"
version = "FortiWeb 7.2"
---
Lists server pools: the real web/application servers behind FortiWeb, their ports, health checks and load balancing method.

## Command

```cisco
show server-pool
```

## Notes

- Verify member IPs and ports first when FortiWeb returns 502/503 style errors.
- Health check configuration per member shows why a server is marked down.
- The balancing method (round robin, least connections) explains unequal traffic distribution.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
