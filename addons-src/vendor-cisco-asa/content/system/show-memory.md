---
id = "cmd/asa/show-memory"
kind = "command"
title = "show memory"
category = "reference/firewalls/cisco-asa/system"
tags = ["asa", "cisco", "cli", "system", "performance"]
risk = "info"
version = "ASA 9.16"
---
Shows total, used and free physical memory plus allocation summary.

## Command

```cisco
show memory
```

## Notes

- The ASA allocator grabs most free memory by design; low free memory alone is not a fault.
- Watch the trend instead: repeated growth without recovery suggests a leak or connection-table growth.
- Use `show memory detail` and `show blocks` to inspect pools when connections are dropped for lack of memory.
- Compare before/after enabling new inspections or logging levels to gauge their cost.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
