---
id = "doc/monitoring/splunk/splunk"
kind = "command"
title = "Splunk SPL"
category = "reference/monitoring/splunk"
tags = ["splunk", "query-anatomy", "core-transforming-commands", "filtering-comparison", "eval-computed-fields", "ip-network-functions-eval", "field-extraction-enrichment", "subsearches-joins", "alerting-detection-patterns", "useful-tokens-gotchas", "model-doc"]
risk = "info"
version = "core-0.3.0"
---
The complete Splunk SPL document: every command for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from src/cheatsheets/splunk.md; each topic is a
section carrying its own tags via a section-tags marker.


## Query anatomy
<!-- section-tags: splunk, query-anatomy -->

```
index=main sourcetype=access_combined status=500
| stats count by clientip
| sort - count
```
Search headless order matters: filters first, then transforming commands, then sorting/formatting.

## Core transforming commands
<!-- section-tags: splunk, core-transforming-commands -->

### stats count by host

Aggregate: count, sum, avg, min, max, dc (distinct count)

```spl
stats count by host
```

Risk: info

### stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri

Percentiles and averages grouped

```spl
stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri
```

Risk: info

### timechart span=5m count by status

Time series, bucketed

```spl
timechart span=5m count by status
```

Risk: info

### chart count over host by sourcetype

Matrix chart

```spl
chart count over host by sourcetype
```

Risk: info

### top limit=10 clientip

Most common values + percentage

```spl
top limit=10 clientip
```

Risk: info

### rare limit=10 uri

Least common values

```spl
rare limit=10 uri
```

Risk: info

### table field1, field2

Keep named fields as columns

```spl
table field1, field2
```

Risk: info

### fields field1, field2

Keep fields (cheaper than table, keeps order for search)

```spl
fields field1, field2
```

Risk: info

### dedup clientip

First event per value

```spl
dedup clientip
```

Risk: info

### sort - _time

Sort descending by field

```spl
sort - _time
```

Risk: info

## Filtering & comparison
<!-- section-tags: splunk, filtering-comparison -->

### index=web status>=500 AND status<600

Range filter in the base search

```spl
index=web status>=500 AND status<600
```

Risk: info

### | where bytes > 10000

Post-aggregation filter

```spl
| where bytes > 10000
```

Risk: info

### | search status=404 OR status=503

Post-filter with OR

```spl
| search status=404 OR status=503
```

Risk: info

### | regex uri="\/api\/.*\.json"

Regex filter on a field

```spl
| regex uri="\/api\/.*\.json"
```

Risk: info

### | match _raw, "(?i)timeout"

Match with a pattern (deprecated — prefer regex)

```spl
| match _raw, "(?i)timeout"
```

Risk: info

## Eval — computed fields
<!-- section-tags: splunk, eval-computed-fields -->

### | eval mb=round(bytes/1024/1024, 2)

Arithmetic

```spl
| eval mb=round(bytes/1024/1024, 2)
```

Risk: info

### | eval size_class=case(bytes<1024,"small", bytes<1048576,"medium", true(),"large")

Conditional buckets

```spl
| eval size_class=case(bytes<1024,"small", bytes<1048576,"medium", true(),"large")
```

Risk: info

### | eval is_error=if(status>=500, 1, 0)

Boolean flag

```spl
| eval is_error=if(status>=500, 1, 0)
```

Risk: info

### | eval cidr=cidrmatch("192.168.0.0/16", clientip)

IP inside a CIDR? (true/false)

```spl
| eval cidr=cidrmatch("192.168.0.0/16", clientip)
```

Risk: info

### | eval user=lower(user)

String functions: lower/upper/trim/substr/len

```spl
| eval user=lower(user)
```

Risk: info

### | eval t=strptime(ts, "%Y-%m-%d %H:%M:%S")

Parse a timestamp

```spl
| eval t=strptime(ts, "%Y-%m-%d %H:%M:%S")
```

Risk: info

### | eval when=strftime(_time, "%F %T")

Format _time

```spl
| eval when=strftime(_time, "%F %T")
```

Risk: info

## IP & network functions (eval)
<!-- section-tags: splunk, ip-network-functions-eval -->

### Function

What it does

```spl
Function
```

Risk: info

### cidrmatch("10.0.0.0/8", ip)

Membership test

```spl
cidrmatch("10.0.0.0/8", ip)
```

Risk: info

### ip("10.1.2.3", "mask")

Netmask of an address

```spl
ip("10.1.2.3", "mask")
```

Risk: info

### ip("10.1.2.3/24", "ip")

Network address

```spl
ip("10.1.2.3/24", "ip")
```

Risk: info

### ip(ip1) == ip(ip2)

Compare as IP, not string

```spl
ip(ip1) == ip(ip2)
```

Risk: info

## Field extraction & enrichment
<!-- section-tags: splunk, field-extraction-enrichment -->

### | rex field=_raw "user=(?<user>\w+)"

Extract a field with regex

```spl
| rex field=_raw "user=(?<user>\w+)"
```

Risk: info

### | rex mode=sed field=_raw "s/token/REDACTED/g"

Sed-style rewrite

```spl
| rex mode=sed field=_raw "s/token/REDACTED/g"
```

Risk: info

### | lookup dns clientip OUTPUT clienthost

Enrich from a lookup table

```spl
| lookup dns clientip OUTPUT clienthost
```

Risk: info

### | fillnull value=0 bytes

Replace nulls

```spl
| fillnull value=0 bytes
```

Risk: info

### | rename clientip AS src_ip

Rename fields

```spl
| rename clientip AS src_ip
```

Risk: info

### | eval-forward

fields stay in pipeline order — extract before you aggregate

```spl
| eval-forward
```

Risk: info

## Subsearches & joins
<!-- section-tags: splunk, subsearches-joins -->

```
index=main [ search index=auth action=failure | top 1 user | fields user ]
```
```
index=main sourcetype=web
| join type=inner session_id [ search index=main sourcetype=auth ]
```
- Subsearches run first and their result becomes a filter — keep them small (use `head`, `top limit=`).
- `join` is expensive; prefer `stats ... by <shared_field>` to correlate.

## Alerting & detection patterns
<!-- section-tags: splunk, alerting-detection-patterns -->

```
index=main sourcetype=auth action=failure
| stats count as fails by user, _time span=10m
| where fails > 20
```
```
index=fw src_ip=*
| stats dc(dest_port) as ports by src_ip
| where ports > 100
```
```
index=web uri=*.php* status=404
| stats count by src_ip, uri
| sort - count
| head 20
```

## Useful tokens & gotchas
<!-- section-tags: splunk, useful-tokens-gotchas -->

- `index=` is mandatory for speed; wildcards on index names are slow.
- `NOT index=_internal` excludes Splunk's own logs.
- Quoted phrases (`"connection reset"`) match literally; unquoted words match independently.
- `_time` filtering (`earliest=-24h latest=now`) happens before the search — always scope it.
- `| head 100` is cheaper than `| sort - _time | head 100` when order doesn't matter.
- `stats` drops fields not listed — carry identifiers with `values(field) as field` or `earliest(field)`.
- `| localop` forces stats to run on the search head (use sparingly).
