---
id = "cmd/paloalto/show-system-info"
kind = "command"
title = "show system info"
category = "reference/firewalls/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "system"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Shows software version, platform, serial number, uptime and management settings.

## Command

```text
show system info
```

## Notes

- The serial number and PAN-OS version are required for TAC cases.
- Also shows management IP, DNS/NTP servers and time of last commit.
- First command to run on an unfamiliar firewall to identify model and release.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
