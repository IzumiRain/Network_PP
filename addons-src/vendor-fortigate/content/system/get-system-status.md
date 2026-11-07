---
id = "cmd/fortigate/get-system-status"
kind = "command"
title = "get system status"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "system"]
risk = "info"
version = "FortiOS 7.4"
---
Shows the appliance summary: firmware version and build, serial number, BIOS, uptime, CPU and memory.

## Command

```cisco
get system status
```

## Notes

- First command to run on any FortiGate session; confirms the firmware build before you apply changes.
- Serial number and uptime are required for most FortiCare/TAC cases.
- Compare the Version line with the images listed by diagnose sys flash list after an upgrade.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
