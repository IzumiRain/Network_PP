---
id = "cmd/mikrotik/traceroute"
kind = "command"
title = "/tool traceroute"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "diagnostics", "traceroute"]
risk = "info"
version = "RouterOS 7.x"
---
Traces the hop-by-hop path from the router to a destination.

## Command

```text
/tool traceroute 8.8.8.8
```

## Notes

- Shows each hop with per-probe latency; timeouts in the middle are often just rate-limited routers.
- `protocol=icmp` helps when the default UDP probes are filtered along the path.
- Run it twice and compare: intermittent paths change between runs.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
