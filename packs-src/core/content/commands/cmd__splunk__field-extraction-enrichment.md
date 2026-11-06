---
id = "cmd/splunk/field-extraction-enrichment"
kind = "command"
title = "\\"
category = "reference/monitoring/splunk/field-extraction-enrichment"
tags = ["splunk", "field-extraction-enrichment"]
risk = "info"
version = "core-0.2.0"
---

**Command:** `\`

rex field=_raw "user=(?<user>\w+)"`

```spl
\
```
