---
id = "cmd/fortiweb/show-waf-ip-reputation"
kind = "command"
title = "show waf ip-reputation"
category = "reference/vendors/fortinet/fortiweb/protection"
tags = ["fortiweb", "waf", "cli", "ip-reputation"]
risk = "info"
version = "FortiWeb 7.2"
---
Prints the IP reputation policies: which FortiGuard IP reputation categories are blocked and the configured action.

## Command

```cisco
show waf ip-reputation
```

## Notes

- Categories cover known attackers, spam sources, anonymizers (VPN/Tor) and similar lists.
- Blocking anonymizer categories can hurt legitimate users behind corporate VPNs — review before enforcing.
- The policy must be attached in the web protection profile to take effect.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
