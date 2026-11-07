---
id = "cmd/fortiauthenticator/show-system-interface"
kind = "command"
title = "show system interface"
category = "reference/vendors/fortinet/fortiauthenticator/system"
tags = ["fortiauthenticator", "identity", "cli", "interfaces"]
risk = "info"
version = "FortiAuthenticator 6.4"
---
Prints interface configuration: IP address, mode, speed/duplex and which management services are allowed.

## Command

```cisco
show system interface
show system interface port1
```

## Notes

- Verify the IP and allowaccess flags when devices cannot reach the RADIUS (1812) or LDAP services.
- Identity appliances often need fixed addressing; DHCP interfaces on an auth server are a red flag.
- Pair with upstream routing checks if remote NAS devices time out against this appliance.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
