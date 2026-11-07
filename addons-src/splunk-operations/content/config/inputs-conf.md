---
id = "ref/splunk/inputs-conf"
kind = "configuration"
title = "inputs.conf basics"
category = "reference/monitoring/splunk-ops/config"
tags = ["splunk", "config", "inputs"]
version = "Splunk Enterprise 9.x"
---
inputs.conf defines what a Splunk instance collects: files, TCP/UDP ports, scripts and Windows sources.

## Example

```ini
[monitor:///var/log/nginx/access.log]
index = web
sourcetype = nginx:access
disabled = 0

[monitor:///var/log/app/*.log]
index = app
sourcetype = app:log
```

## Notes

- Stanzas live in `$SPLUNK_HOME/etc/system/local/inputs.conf` or app-scoped under `etc/apps/<app>/local/` - app-scoped is the deployable pattern.
- `index` must exist on the receiving indexers or data lands in `main`/fails; validate with `btool indexes list`.
- Set `sourcetype` explicitly at collection time - it anchors all later props.conf parsing.
- `disabled = 1` silences an input without deleting the stanza; useful during migrations.
- The CLI (`splunk add monitor /path`) is a convenient generator for the same stanzas.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
