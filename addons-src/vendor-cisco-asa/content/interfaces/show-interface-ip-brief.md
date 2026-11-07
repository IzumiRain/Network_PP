---
id = "cmd/asa/show-interface-ip-brief"
kind = "command"
title = "show interface ip brief"
category = "reference/vendors/cisco/asa/interfaces"
tags = ["asa", "cisco", "cli", "interfaces"]
risk = "info"
version = "ASA 9.16"
---
One-line-per-interface summary of IP address, status and line protocol state.

## Command

```cisco
show interface ip brief
```

## Notes

- `up/up` means administratively up and carrier detected; `up/down` usually means wrong speed/duplex or no peer.
- Use for a quick health pass before deeper checks like `show interface GigabitEthernet0/1` (counters, errors).
- Subinterfaces appear with their VLAN tags; a missing dot1Q tag is a common cause of `up/down`.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
