---
id = "cmd/paloalto/request-restart-system"
kind = "command"
title = "request restart system"
category = "reference/firewalls/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "reboot"]
risk = "high"
version = "PAN-OS 10.x/11.x"
---
Reboots the firewall, taking management and data plane down until it returns.

## Command

```text
request restart system
```

## Notes

- Full outage for all traffic inspected by this device - schedule a maintenance window.
- In an HA pair, fail over to the peer first so the secondary reboots instead of the active.
- A dataplane-only restart also exists (`request restart dataplane`) but it drops traffic too.
- Confirm no commit or upgrade job is running with `show jobs all` before restarting.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
