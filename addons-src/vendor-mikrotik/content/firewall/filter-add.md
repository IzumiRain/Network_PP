---
id = "cmd/mikrotik/ip-firewall-filter-add"
kind = "command"
title = "/ip firewall filter add"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "firewall"]
risk = "medium"
version = "RouterOS 7.x"
---
Adds a firewall filter rule; rules take effect immediately in top-down order.

## Command

```text
/ip firewall filter add chain=input protocol=tcp dst-port=22 src-address=10.0.0.0/24 action=accept
/ip firewall filter disable 3
```

## Notes

- The example allows SSH from one subnet only; adapt chain, protocol and ports to your policy.
- A wrong early drop rule can cut your own management session - keep console access available.
- Test with `disable`/`enable <rule-number>` before committing to a change.
- Keep established/related accept rules ordered before catch-all drops.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
