---
id = "cmd/asa/show-clock"
kind = "command"
title = "show clock"
category = "reference/vendors/cisco/asa/system"
tags = ["asa", "cisco", "cli", "system", "ntp"]
risk = "info"
version = "ASA 9.16"
---
Shows the firewall's current date, time and timezone; `detail` adds the time source.

## Command

```cisco
show clock
show clock detail
```

## Notes

- Wrong clocks break log correlation, certificate validity checks and VPN rekeys.
- `show clock detail` shows whether the time is authoritative or synced from NTP.
- Check `ntp server` / `show ntp status` when the clock drifts or shows an NTP fallback source.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
