---
id = "cmd/fortigate/get-vpn-ipsec-tunnel-summary"
kind = "command"
title = "get vpn ipsec tunnel summary"
category = "reference/firewalls/fortigate/vpn"
tags = ["fortigate", "fortios", "cli", "ipsec"]
risk = "info"
version = "FortiOS 7.4"
---
Prints a compact one-line status per IPsec tunnel: tunnel name, state (up/down) and active SAs.

## Command

```cisco
get vpn ipsec tunnel summary
```

## Notes

- Fastest health overview for scripts and shift-handover notes, versus the verbose diagnose vpn tunnel list.
- A tunnel can show up while no traffic passes; confirm with counters before declaring it healthy.
- Combine with get system ha status on HA clusters to see which member owns the tunnel.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
