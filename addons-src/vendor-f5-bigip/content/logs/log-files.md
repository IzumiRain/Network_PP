---
id = "ref/f5-bigip/log-files"
kind = "configuration"
title = "Log files and logging"
category = "reference/vendors/f5/bigip/logs"
tags = ["f5", "bigip", "logs"]
version = "BIG-IP 16.1"
---
BIG-IP writes component logs to /var/log via syslogd; they are read over SSH (advanced shell) rather than through tmsh.

## Key files

```bash
tail -f /var/log/ltm    # LTM events: monitor state changes, connection notices
tail -f /var/log/audit  # config changes via tmsh/GUI
tail -f /var/log/asm    # WAF (ASM) events
tail -f /var/log/gtm    # DNS/GTM events
```

## Notes

- /var/log/ltm is the first place to look for monitor flaps and connection-level errors.
- For permanent retention, configure remote high-speed logging (HSL) or syslog destinations in the GUI.
- Log disk usage matters: archive or offload rather than letting /var fill up on the appliance.

Source: F5 BIG-IP System Logging Guide — https://techdocs.f5.com
