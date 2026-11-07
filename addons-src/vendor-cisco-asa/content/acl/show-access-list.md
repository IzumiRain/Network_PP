---
id = "cmd/asa/show-access-list"
kind = "command"
title = "show access-list"
category = "reference/vendors/cisco/asa/acl"
tags = ["asa", "cisco", "cli", "acl"]
risk = "info"
version = "ASA 9.16"
---
Shows ACL entries with per-ACE hit counters - the ASA firewall policy in effect.

## Command

```cisco
show access-list
show access-list INSIDE_IN
```

## Notes

- Hit counts tell you whether a rule ever matched; a silent rule above a general permit may be dead config.
- The implicit `deny any any` at the end does not show a counter - use `packet-tracer` to prove a deny.
- Crypto ACLs for old-style L2L VPNs also appear here; their hits come through the tunnel, not the interface.
- `show run access-list` shows the configured text, while this shows counters with expanded line numbers.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
