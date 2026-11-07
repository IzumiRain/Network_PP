---
id = "cmd/mikrotik/ping"
kind = "command"
title = "/ping"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "diagnostics", "ping"]
risk = "info"
version = "RouterOS 7.x"
---
Sends ICMP echo requests from the router; ping is a top-level command.

## Command

```text
/ping 8.8.8.8
/ping 8.8.8.8 count=10 size=1400
```

## Notes

- Runs until stopped with Ctrl+C unless you pass `count=`.
- Use `size=` near path-MTU limits to hunt fragmentation problems.
- `src-address=` tests reachability from a specific local IP (useful with multiple WAN links).
- Note that traceroute lives under `/tool traceroute`, not `/tool ping`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
