---
id = "cmd/windows/get-process-id-get-nettcpconnection-lo"
kind = "command"
title = "Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess"
category = "reference/os/windows/processes-services"
tags = ["windows", "processes-services"]
risk = "low"
version = "core-0.2.0"
---

**Command:** `Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess`

Process owning a port

```powershell
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess
```
