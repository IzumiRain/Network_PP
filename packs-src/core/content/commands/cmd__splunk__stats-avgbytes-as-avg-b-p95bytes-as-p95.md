---
id = "cmd/splunk/stats-avgbytes-as-avg-b-p95bytes-as-p95"
kind = "command"
title = "stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri"
category = "reference/monitoring/splunk/core-transforming-commands"
tags = ["splunk", "core-transforming-commands"]
risk = "info"
version = "core-0.2.0"
---

**Command:** `stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri`

Percentiles and averages grouped

```spl
stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri
```
