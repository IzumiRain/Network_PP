---
id = "cmd/mikrotik/ip-address-print"
kind = "command"
title = "/ip address print"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "ip", "interfaces"]
risk = "info"
version = "RouterOS 7.x"
---
Shows every IP address configured on the router with its interface and status.

## Command

```text
/ip address print
/ip address print detail
```

## Notes

- `A` flag marks the active address; `D` marks dynamic ones (DHCP client, PPP).
- Shows address, network, interface and interface link state in one view.
- Addresses are managed in the same submenu - expect a brief outage when changing the address you are connected through.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
