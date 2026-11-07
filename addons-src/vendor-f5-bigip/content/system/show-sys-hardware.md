---
id = "cmd/f5-bigip/show-sys-hardware"
kind = "command"
title = "show sys hardware"
category = "reference/vendors/f5/bigip/system"
tags = ["f5", "bigip", "tmsh", "hardware"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows platform hardware details: chassis model, CPU, memory, and fan/temperature/power sensor readings.

## Command

```bash
tmsh show sys hardware
```

## Notes

- Sensor failures (fans, PSU, temperature) predict unplanned failovers — check this before and after maintenance windows.
- Confirms the exact platform model when ordering parts or validating license/platform compatibility.
- Read-only and safe to run at any time, including on active units.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
