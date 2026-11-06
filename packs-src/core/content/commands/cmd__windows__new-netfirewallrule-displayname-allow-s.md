---
id = "cmd/windows/new-netfirewallrule-displayname-allow-s"
kind = "command"
title = "New-NetFirewallRule -DisplayName \"Allow SSH\" -Direction Inbound -Protocol TCP -LocalPort 2"
category = "reference/os/windows/firewall"
tags = ["windows", "firewall"]
risk = "low"
version = "core-0.2.0"
---

**Command:** `New-NetFirewallRule -DisplayName "Allow SSH" -Direction Inbound -Protocol TCP -LocalPort 22 -Action Allow`

Add an inbound allow rule

```powershell
New-NetFirewallRule -DisplayName "Allow SSH" -Direction Inbound -Protocol TCP -LocalPort 22 -Action Allow
```
