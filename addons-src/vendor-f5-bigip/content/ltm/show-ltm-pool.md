---
id = "cmd/f5-bigip/show-ltm-pool"
kind = "command"
title = "show ltm pool"
category = "reference/devices/f5-bigip/ltm"
tags = ["f5", "bigip", "tmsh", "ltm"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows runtime pool statistics and per-member status: availability (up/down), current connections and bytes.

## Command

```bash
tmsh show ltm pool
tmsh show ltm pool /Common/pool-app members
```

## Notes

- The members detail view is the go-to for member status: up (green), down, checking or disabled.
- Members flagged down by a monitor will not receive new connections even though the virtual is up.
- Combine with list ltm pool to separate configuration from runtime state when a member keeps flapping.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
