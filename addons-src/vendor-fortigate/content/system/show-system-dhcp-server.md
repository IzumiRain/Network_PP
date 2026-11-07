---
id = "cmd/fortigate/show-system-dhcp-server"
kind = "command"
title = "show system dhcp server"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "dhcp"]
risk = "info"
version = "FortiOS 7.4"
---
Lists the DHCP server configuration per interface: ranges, lease time, gateway and DNS options.

## Command

```cisco
show system dhcp server
```

## Notes

- Shows the start/end IP ranges, netmask, default-gateway and dns-server handed out on each interface.
- Verify range boundaries and reserved (MAC-mapped) IPs when clients receive wrong or overlapping addresses.
- The CLI shows configuration only; active lease state is inspected in the GUI or via the dhcpd daemon.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
