---
id = "cmd/mikrotik/ip-hotspot-print"
kind = "command"
title = "/ip hotspot print (servers and users)"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "hotspot", "captive-portal"]
risk = "info"
version = "RouterOS 7.x"
---
Shows HotSpot server interfaces, user accounts and currently active sessions.

## Command

```text
/ip hotspot print
/ip hotspot user print
/ip hotspot active print
```

## Notes

- `ip hotspot` lists HotSpot servers bound to interfaces; `ip hotspot user` lists accounts and quotas.
- `ip hotspot active` shows users that are currently authenticated, with IPs and uptime.
- Login-page and RADIUS settings live in `ip hotspot profile` and `ip hotspot user profile`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
