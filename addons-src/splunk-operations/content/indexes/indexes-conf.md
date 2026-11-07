---
id = "ref/splunk/indexes-conf"
kind = "configuration"
title = "indexes.conf basics"
category = "reference/monitoring/splunk-ops/indexes"
tags = ["splunk", "config", "indexes"]
version = "Splunk Enterprise 9.x"
---
indexes.conf defines indexes: storage paths, size caps and retention behavior.

## Example

```ini
[web_data]
homePath = $SPLUNK_DB/web_data/db
coldPath = $SPLUNK_DB/web_data/colddb
thawedPath = $SPLUNK_DB/web_data/thaweddb
maxTotalDataSizeMB = 512000
```

## Notes

- homePath, coldPath and thawedPath are mandatory for every index stanza - a missing path breaks index creation.
- Deploy indexes as an app to indexers (and cluster manager in a cluster); applying only on peers causes manager/peer drift.
- maxTotalDataSizeMB and frozenTimePeriodInSecs drive retention; without a frozenPath, archived buckets are deleted.
- `splunk btool indexes list --debug` shows the effective merged definition including defaults per index.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
