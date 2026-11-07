---
id = "cmd/fortigate/diag-hardware-deviceinfo-nic"
kind = "command"
title = "diagnose hardware deviceinfo nic"
category = "reference/firewalls/fortigate/interfaces"
tags = ["fortigate", "fortios", "cli", "interfaces"]
risk = "info"
version = "FortiOS 7.4"
---
Shows driver-level NIC details: link speed and duplex, media, MAC and packet/error counters for the named interface.

## Command

```cisco
diagnose hardware deviceinfo nic port1
diagnose hardware deviceinfo nic
```

## Notes

- Run with an interface name for one NIC; without a name it lists every NIC on the box.
- Link down here while show system interface still shows the IP configured points to cabling or switch port problems.
- Rising error/drop counters usually mean duplex mismatch, a bad cable, or an overloaded interface.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
