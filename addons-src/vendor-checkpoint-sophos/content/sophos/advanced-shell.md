---
id = "cmd/sophos/advanced-shell"
kind = "command"
title = "advanced shell (SFOS)"
category = "reference/vendors/sophos"
tags = ["sophos", "sfos", "cli", "shell"]
risk = "high"
version = "Sophos Firewall OS 19.x+"
---
Enters the Linux advanced shell from the SFOS console for low-level troubleshooting.

## Command

```text
sudo
```

## Notes

- Drops into a Linux shell used only for vendor-documented procedures and support-driven steps.
- Changes made here can break the appliance and are typically unsupported - document what you do.
- The exact access method (sudo with the admin password, or support-enabled accounts) varies by SFOS release - verify in the current CLI guide.
- Leave the shell promptly and confirm normal services afterwards.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com
