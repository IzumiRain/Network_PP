---
id = "cmd/f5-bigip/show-sys-conn"
kind = "command"
title = "show sys conn"
category = "reference/devices/f5-bigip/connections"
tags = ["f5", "bigip", "tmsh", "connections"]
risk = "info"
version = "BIG-IP 16.1"
---
Dumps the live connection table: client and server side endpoints, protocol and connection state.

## Command

```bash
tmsh show sys conn
tmsh show sys conn | grep 10.10.10.5
```

## Notes

- Shows client-side (cs) and server-side (ss) tuples so you can trace one flow end-to-end through the BIG-IP.
- Pipe through grep to filter for a client IP or virtual server; the full table is large on busy units.
- Stuck connections in an abnormal state point at idle timeouts, monitors, or a misconfigured SNAT causing asymmetry.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
