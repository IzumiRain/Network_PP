---
id = "cmd/windows/get-winevent-filterhashtable-lognamesys"
kind = "command"
title = "Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24"
category = "reference/os/windows/system-info-events"
tags = ["windows", "system-info-events"]
risk = "low"
version = "core-0.2.0"
---

**Command:** `Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24)}`

Errors from last 24h (modern)

```powershell
Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24)}
```
