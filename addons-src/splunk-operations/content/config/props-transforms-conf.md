---
id = "ref/splunk/props-transforms-conf"
kind = "configuration"
title = "props.conf and transforms.conf basics"
category = "reference/monitoring/splunk-ops/config"
tags = ["splunk", "config", "props", "transforms"]
version = "Splunk Enterprise 9.x"
---
props.conf applies per-source/sourcetype settings (line breaking, timestamps); transforms.conf implements regex transformations it references.

## Example

```ini
# props.conf
[nginx:access]
SHOULD_LINEMERGE = false
LINE_BREAKER = ([\r\n]+)
TIME_PREFIX = ^\[
TIME_FORMAT = %d/%b/%Y:%H:%M:%S %z
MAX_TIMESTAMP_LOOKAHEAD = 25
TRUNCATE = 10000
```

```ini
# transforms.conf (masking pattern)
[mask_account_number]
REGEX = (?m)^(.*)\d{12}(\d{4})(.*)$
FORMAT = $1XXXXXXXXXXXX$2$3
DEST_KEY = _raw
```

## Notes

- Index-time props (LINE_BREAKER, TIME_*, TRUNCATE) affect data as it is parsed - test on a lab indexer before rolling out; bad settings cannot be un-applied to already-indexed data.
- transforms.conf entries are referenced from props.conf via REPORT-, TRANSFORMS- or SEDCMD-style keys for extractions, masks and routing.
- Keep both files together in one app so the deployable unit is self-contained.
- Search-time extractions (REPORT-/EXTRACT-) are safer than index-time fields; prefer them unless index-time is required.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
