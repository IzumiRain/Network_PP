---
id = "cmd/asa/show-version"
kind = "command"
title = "show version"
category = "reference/vendors/cisco/asa/system"
tags = ["asa", "cisco", "cli", "system"]
risk = "info"
version = "ASA 9.16"
---
Shows the ASA software image, uptime, serial number, licenses and a summary of flash contents.

## Command

```cisco
show version
```

## Notes

- Confirms the exact running image and platform before upgrades or a TAC case.
- Lists the chassis serial number and activation-key licenses (Security Plus, AnyConnect counts).
- Uptime and last reload reason appear here; pair with `show crashinfo` after an unexpected reboot.
- On Firepower (Secure Firewall) appliances, the FXOS version is shown alongside the ASA version.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
