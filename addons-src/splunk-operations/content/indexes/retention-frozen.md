---
id = "ref/splunk/retention-frozen"
kind = "configuration"
title = "Index retention: frozenTimePeriodInSecs and frozen data"
category = "reference/monitoring/splunk-ops/indexes"
tags = ["splunk", "indexes", "retention"]
version = "Splunk Enterprise 9.x"
---
Data ages out when it is older than frozenTimePeriodInSecs or when size caps are exceeded; frozen data is deleted unless an archive path is set.

## Example

```ini
[web_data]
frozenTimePeriodInSecs = 7776000      # 90 days
maxTotalDataSizeMB = 512000
# coldToFrozenDir = /archive/web_data   # uncomment to archive instead of delete
```

## Notes

- Default frozenTimePeriodInSecs is roughly 6 years (188697600); set it explicitly per index to a deliberate value.
- Retention is evaluated per bucket by its newest event, not per individual event - recent data shares the bucket age.
- With no frozen destination configured (default), buckets reaching the criteria are deleted from disk.
- Size-based retention (maxTotalDataSizeMB / maxDataSize) can remove data earlier than the time policy when ingest is heavy.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
