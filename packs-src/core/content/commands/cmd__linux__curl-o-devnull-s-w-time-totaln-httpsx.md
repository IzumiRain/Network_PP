---
id = "cmd/linux/curl-o-devnull-s-w-time-totaln-httpsx"
kind = "command"
title = "curl -o /dev/null -s -w '%{time_total}\\n' https://x.com"
category = "reference/os/linux/performance-quick-probes"
tags = ["linux", "performance-quick-probes"]
risk = "low"
version = "core-0.2.0"
---

**Command:** `curl -o /dev/null -s -w '%{time_total}\n' https://x.com`

Total request time

```bash
curl -o /dev/null -s -w '%{time_total}\n' https://x.com
```
