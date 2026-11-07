---
id = "ref/splunk/deploymentclient-conf"
kind = "configuration"
title = "deploymentclient.conf (forwarder side)"
category = "reference/monitoring/splunk-ops/deployment"
tags = ["splunk", "deployment", "forwarder-management"]
version = "Splunk Enterprise 9.x"
---
deploymentclient.conf points a forwarder at its deployment server for app management.

## Example

```ini
[deploymentClient]
targetUri = deploy01.example.com:8089

[target-broker:deploymentClient]
targetUri = deploy01.example.com:8089
```

## Notes

- Target port is the management port 8089, not the data port 9997.
- Once managed, apps the deployment server owns are reinstalled if removed locally - fix drift at the server, not on the forwarder.
- `phoneHomeIntervalInSecs` tunes polling frequency; lowering it helps during migrations, keep it moderate at scale.
- Verify management connectivity first: `telnet deploy01.example.com 8089` / `splunk btool deploymentclient list --debug`.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
