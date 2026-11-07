---
id = "cmd/pfsense/config-backup"
kind = "command"
title = "back up /conf/config.xml"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "cli", "backup", "config"]
risk = "low"
version = "pfSense CE 2.7 / OPNsense 24.x"
---
Copies the complete firewall configuration file to a local backup path.

## Command

```bash
cp /conf/config.xml /root/config-backup-$(date +%F).xml
```

## Notes

- config.xml holds the full configuration including secrets - store copies securely.
- Get it off-box with `scp admin@<firewall>:/conf/config.xml .`.
- Restore by uploading via the GUI (Diagnostics > Backup & Restore) or replacing the file and rebooting.
- Both pfSense and OPNsense keep the active config at /conf/config.xml.

Source: Netgate pfSense Documentation — https://docs.netgate.com
