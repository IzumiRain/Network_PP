---
id = "cmd/fortigate/show-system-interface"
kind = "command"
title = "show system interface"
category = "reference/vendors/fortinet/fortigate/interfaces"
tags = ["fortigate", "fortios", "cli", "interfaces"]
risk = "info"
version = "FortiOS 7.4"
---
Prints the layer-3 configuration of every interface: mode and address, speed/duplex, role, MTU and management access.

## Command

```cisco
show system interface
show system interface port1
```

## Notes

- Check mode (static vs dhcp), the IP/mask and secondary IPs first when an interface looks wrong.
- allowaccess lists which management services (ping https ssh snmp) are open on the interface.
- Pair with diagnose hardware deviceinfo nic for the physical link state underneath.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
