---
id = "cmd/juniper/show-route"
kind = "command"
title = "show route"
category = "reference/vendors/juniper"
tags = ["juniper", "junos", "cli", "routing"]
risk = "info"
version = "Junos OS 22.x"
---
Displays the Junos routing tables, by default the IPv4 unicast table inet.0.

## Command

```text
show route
show route protocol ospf
show route 10.0.0.0/8
```

## Notes

- The active route for each prefix is marked with `*`.
- Filter by protocol (ospf, bgp, static, direct), by prefix, or use `show route terse` for a compact view.
- IPv6 routes live in table inet6.0: `show route table inet6.0`.
- Routes shown as `Unusable` or hidden indicate a failed next-hop resolution.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
