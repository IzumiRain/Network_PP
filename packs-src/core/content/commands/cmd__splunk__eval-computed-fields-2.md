---
id = "cmd/splunk/eval-computed-fields-2"
kind = "command"
title = "\\"
category = "reference/monitoring/splunk/eval-computed-fields"
tags = ["splunk", "eval-computed-fields"]
risk = "info"
version = "core-0.2.0"
---

**Command:** `\`

eval size_class=case(bytes<1024,"small", bytes<1048576,"medium", true(),"large")`

```spl
\
```
