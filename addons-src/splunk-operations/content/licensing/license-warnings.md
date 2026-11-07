---
id = "ref/splunk/license-warnings"
kind = "protocol"
title = "License violations and warnings"
category = "reference/monitoring/splunk-ops/licensing"
tags = ["splunk", "licensing", "operations"]
version = "Splunk Enterprise 9.x"
---
Exceeding a daily license quota logs warnings; enough warnings within a window disables search until resolved.

## Example

```text
index=_internal source=*license_usage.log type=Usage
```

## Notes

- Data exceeding the quota is still indexed, but the event triggers a license warning - ingestion does not stop.
- More than 5 warnings in a rolling 30-day window disables search until the window clears or licensing is fixed.
- Check the daily usage query (`type=Usage`) to find which index/host drove the overage before deciding on a pool change.
- Temporary licenses and emergency/capacity changes behave differently - always review the current license terms before relying on behavior.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
