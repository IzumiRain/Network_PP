---
id = "cmd/paloalto/show-interface-all"
kind = "command"
title = "show interface all"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "interfaces"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Lists all interfaces with virtual router, zone, type and link state.

## Command

```text
show interface all
show interface ethernet1/1
```

## Notes

- State `up` is operational, `down` means no link, `n/a` means not configured or not attached.
- The name-filtered form shows IPs, counters and detail for a single interface.
- Verify each interface landed in the intended virtual router and zone.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
