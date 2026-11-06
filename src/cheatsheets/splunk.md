# Splunk SPL cheat sheet

## Query anatomy

```
index=main sourcetype=access_combined status=500
| stats count by clientip
| sort - count
```

Search headless order matters: filters first, then transforming commands, then sorting/formatting.

## Core transforming commands

| Command | What it does |
|---|---|
| `stats count by host` | Aggregate: count, sum, avg, min, max, dc (distinct count) |
| `stats avg(bytes) as avg_b, p95(bytes) as p95_b by uri` | Percentiles and averages grouped |
| `timechart span=5m count by status` | Time series, bucketed |
| `chart count over host by sourcetype` | Matrix chart |
| `top limit=10 clientip` | Most common values + percentage |
| `rare limit=10 uri` | Least common values |
| `table field1, field2` | Keep named fields as columns |
| `fields field1, field2` | Keep fields (cheaper than table, keeps order for search) |
| `dedup clientip` | First event per value |
| `sort - _time` | Sort descending by field |

## Filtering & comparison

| Command | What it does |
|---|---|
| `index=web status>=500 AND status<600` | Range filter in the base search |
| `\| where bytes > 10000` | Post-aggregation filter |
| `\| search status=404 OR status=503` | Post-filter with OR |
| `\| regex uri="\/api\/.*\.json"` | Regex filter on a field |
| `\| match _raw, "(?i)timeout"` | Match with a pattern (deprecated — prefer regex) |

## Eval — computed fields

| Command | What it does |
|---|---|
| `\| eval mb=round(bytes/1024/1024, 2)` | Arithmetic |
| `\| eval size_class=case(bytes<1024,"small", bytes<1048576,"medium", true(),"large")` | Conditional buckets |
| `\| eval is_error=if(status>=500, 1, 0)` | Boolean flag |
| `\| eval cidr=cidrmatch("192.168.0.0/16", clientip)` | IP inside a CIDR? (true/false) |
| `\| eval user=lower(user)` | String functions: lower/upper/trim/substr/len |
| `\| eval t=strptime(ts, "%Y-%m-%d %H:%M:%S")` | Parse a timestamp |
| `\| eval when=strftime(_time, "%F %T")` | Format _time |

## IP & network functions (eval)

| Function | What it does |
|---|---|
| `cidrmatch("10.0.0.0/8", ip)` | Membership test |
| `ip("10.1.2.3", "mask")` | Netmask of an address |
| `ip("10.1.2.3/24", "ip")` | Network address |
| `ip(ip1) == ip(ip2)` | Compare as IP, not string |

## Field extraction & enrichment

| Command | What it does |
|---|---|
| `\| rex field=_raw "user=(?<user>\w+)"` | Extract a field with regex |
| `\| rex mode=sed field=_raw "s/token/REDACTED/g"` | Sed-style rewrite |
| `\| lookup dns clientip OUTPUT clienthost` | Enrich from a lookup table |
| `\| fillnull value=0 bytes` | Replace nulls |
| `\| rename clientip AS src_ip` | Rename fields |
| `\| eval-forward` fields stay in pipeline order — extract before you aggregate |

## Subsearches & joins

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

- `index=` is mandatory for speed; wildcards on index names are slow.
- `NOT index=_internal` excludes Splunk's own logs.
- Quoted phrases (`"connection reset"`) match literally; unquoted words match independently.
- `_time` filtering (`earliest=-24h latest=now`) happens before the search — always scope it.
- `| head 100` is cheaper than `| sort - _time | head 100` when order doesn't matter.
- `stats` drops fields not listed — carry identifiers with `values(field) as field` or `earliest(field)`.
- `| localop` forces stats to run on the search head (use sparingly).
