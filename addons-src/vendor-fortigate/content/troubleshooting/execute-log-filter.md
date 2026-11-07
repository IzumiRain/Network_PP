---
id = "cmd/fortigate/execute-log-filter"
kind = "command"
title = "execute log filter"
category = "reference/firewalls/fortigate/troubleshooting"
tags = ["fortigate", "fortios", "cli", "logging"]
risk = "low"
version = "FortiOS 7.4"
---
Sets the criteria (log category, device, date range, free-text) that execute log display will match.

## Command

```cisco
execute log filter category 1
execute log filter free-text 10.0.0.5
```

## Notes

- Category 1 is traffic and category 2 is event; see the log category reference for the full numbering.
- The filter persists for later display runs, so reset or re-set it between searches to avoid stale criteria.
- Changing the filter only affects CLI log retrieval, not what the device actually logs.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
