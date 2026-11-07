---
id = "cmd/f5-bigip/show-ltm-virtual"
kind = "command"
title = "show ltm virtual"
category = "reference/vendors/f5/bigip/ltm"
tags = ["f5", "bigip", "tmsh", "ltm"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows runtime statistics per virtual server: current connections, throughput and packet counters.

## Command

```bash
tmsh show ltm virtual
tmsh show ltm virtual /Common/vs-app-443
```

## Notes

- Zero current connections on a supposedly busy virtual points to monitor failures or upstream routing.
- Growing counters confirm traffic actually arrives; flat counters mean the path (or the VIP) is broken.
- Status (Availability/State) lines tell you whether the virtual itself is enabled and able to accept traffic.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
