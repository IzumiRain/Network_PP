---
id = "ref/splunk/deployment-server-serverclass"
kind = "configuration"
title = "Deployment server and serverclass.conf"
category = "reference/monitoring/splunk-ops/deployment"
tags = ["splunk", "deployment", "forwarder-management"]
version = "Splunk Enterprise 9.x"
---
The deployment server pushes apps to forwarders; serverclass.conf maps which apps go to which clients.

## Example

```ini
# serverclass.conf
[serverClass:linux_forwarders]
whitelist.0 = srv-linux-*
[serverClass:linux_forwarders:app:base_inputs_linux]
```

## Notes

- Apps to be distributed live in `$SPLUNK_HOME/etc/deployment-apps/`; serverclass targets them by whitelist/blacklist or machine types.
- Clients poll the server on a configured interval (phoneHomeIntervalInSecs on the forwarder side) and download app changes automatically.
- App state changes take effect on the client at next poll or restart, depending on app configuration.
- Push changes with `splunk reload deploy-server` after editing serverclass.conf; validate mappings with `splunk show deploy-server` style checks or the UI.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
