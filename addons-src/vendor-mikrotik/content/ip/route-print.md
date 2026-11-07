---
id = "cmd/mikrotik/ip-route-print"
kind = "command"
title = "/ip route print"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "routing"]
risk = "info"
version = "RouterOS 7.x"
---
Displays the routing table with active and dynamic route flags.

## Command

```text
/ip route print
/ip route print where dst-address=10.0.0.0/8
```

## Notes

- `A` marks routes actually in use; `D` marks dynamic entries (connected, DHCP, OSPF/BGP learned).
- `0.0.0.0/0` entries are default routes - verify the expected gateway is active.
- `print where` filters (e.g. by `gateway` or `routing-table`); `print detail` shows all attributes.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
