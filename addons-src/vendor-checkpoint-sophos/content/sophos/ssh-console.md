---
id = "cmd/sophos/ssh-console"
kind = "command"
title = "SSH console access (SFOS)"
category = "reference/vendors/sophos"
tags = ["sophos", "sfos", "cli"]
risk = "info"
version = "Sophos Firewall OS 19.x+"
---
Opens the Sophos Firewall CLI console over SSH with an administrator account.

## Command

```bash
ssh admin@<firewall-ip>
```

## Notes

- The SSH console is the primary CLI; log in on TCP 22 with an admin user.
- The prompt accepts SFOS CLI commands (system, network, and related families).
- SSH access per zone is controlled under Administration > Device access.
- Some tasks are WebAdmin-only; the CLI is intentionally limited.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com
