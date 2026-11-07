---
id = "cmd/fortiweb/get-system-status"
kind = "command"
title = "get system status"
category = "reference/vendors/fortinet/fortiweb/system"
tags = ["fortiweb", "waf", "cli"]
risk = "info"
version = "FortiWeb 7.2"
---
Shows the FortiWeb appliance summary: firmware version, model, serial number and uptime.

## Command

```cisco
get system status
```

## Notes

- First command on any CLI session; confirms firmware build before you touch WAF policies.
- Serial number and version are required when opening FortiCare cases.
- Check that signature updates are flowing after a network change (dashboard or update log).

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
