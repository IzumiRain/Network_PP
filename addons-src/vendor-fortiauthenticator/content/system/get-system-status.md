---
id = "cmd/fortiauthenticator/get-system-status"
kind = "command"
title = "get system status"
category = "reference/vendors/fortinet/fortiauthenticator/system"
tags = ["fortiauthenticator", "identity", "cli"]
risk = "info"
version = "FortiAuthenticator 6.4"
---
Shows the FortiAuthenticator appliance summary: firmware version, model, serial number and uptime.

## Command

```cisco
get system status
```

## Notes

- Confirms firmware build and licensing state before you change identity services.
- Serial number is required for FortiCare cases and cluster member identification.
- An identity appliance outage takes down MFA for everything behind it, so check status after reboots first.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
