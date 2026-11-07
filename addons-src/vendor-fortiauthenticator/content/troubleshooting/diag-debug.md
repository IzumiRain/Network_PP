---
id = "cmd/fortiauthenticator/diag-debug"
kind = "command"
title = "diagnose debug enable and disable"
category = "reference/vendors/fortinet/fortiauthenticator/troubleshooting"
tags = ["fortiauthenticator", "identity", "cli", "debug"]
risk = "info"
version = "FortiAuthenticator 6.4"
---
Toggles CLI debug output for the FortiOS-style diagnose debug commands on the appliance.

## Command

```cisco
diagnose debug enable
diagnose debug disable
```

## Notes

- Enable debug output before running targeted debug commands, then disable immediately afterwards.
- Most day-to-day FAC troubleshooting uses the GUI: live logs under Logging, RADIUS/LDAP failure counters and the certificate pages.
- Debug sessions on a busy identity appliance add CPU load; avoid leaving them running.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
