---
id = "cmd/fortigate/diag-vpn-tunnel-list"
kind = "command"
title = "diagnose vpn tunnel list"
category = "reference/firewalls/fortigate/vpn"
tags = ["fortigate", "fortios", "cli", "ipsec"]
risk = "info"
version = "FortiOS 7.4"
---
Lists every IPsec tunnel with phase1/phase2 SA details, lifetimes, and rx/tx byte counters.

## Command

```cisco
diagnose vpn tunnel list
```

## Notes

- Byte counters growing on both sides means traffic is actually flowing through the tunnel.
- Counters stuck while the tunnel reports up usually indicate a phase2 selector or routing problem.
- Pair with diagnose vpn ike gateway list for the phase1 peer negotiation state.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
