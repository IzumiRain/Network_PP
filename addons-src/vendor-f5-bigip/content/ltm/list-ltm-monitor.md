---
id = "cmd/f5-bigip/list-ltm-monitor"
kind = "command"
title = "list ltm monitor"
category = "reference/devices/f5-bigip/ltm"
tags = ["f5", "bigip", "tmsh", "ltm", "monitors"]
risk = "info"
version = "BIG-IP 16.1"
---
Lists the health monitor definitions (http, https, tcp, gateway-icmp and more) with intervals, timeouts and send/receive strings.

## Command

```bash
tmsh list ltm monitor
tmsh list ltm monitor http /Common/http-basic
```

## Notes

- Interval and timeout decide how fast a dead member is pulled: interval 5 / timeout 16 is a common pattern.
- For HTTP monitors the receive-disable-string marks a member available but not in service (drain state).
- A receive string that never matches (wrong status code expected) keeps members down — test the probe manually.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
