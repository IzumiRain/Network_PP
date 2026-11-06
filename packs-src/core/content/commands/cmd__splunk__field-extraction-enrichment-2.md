---
id = "cmd/splunk/field-extraction-enrichment-2"
kind = "command"
title = "\\"
category = "reference/monitoring/splunk/field-extraction-enrichment"
tags = ["splunk", "field-extraction-enrichment"]
risk = "info"
version = "core-0.2.0"
---

**Command:** `\`

rex mode=sed field=_raw "s/token/REDACTED/g"`

```spl
\
```
