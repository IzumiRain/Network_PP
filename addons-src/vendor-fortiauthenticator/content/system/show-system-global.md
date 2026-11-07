---
id = "cmd/fortiauthenticator/show-system-global"
kind = "command"
title = "show system global"
category = "reference/security/fortiauthenticator/system"
tags = ["fortiauthenticator", "identity", "cli"]
risk = "info"
version = "FortiAuthenticator 6.4"
---
Dumps the global system configuration: hostname, admin settings, DNS and NTP-related platform options.

## Command

```cisco
show system global
```

## Notes

- Correct NTP and DNS matter more than usual here: certificates, SAML and OTP all depend on accurate time.
- Check the admin HTTPS port and timeout before scripted or firewalled GUI access.
- Read-only view; changes are made under config system global.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
