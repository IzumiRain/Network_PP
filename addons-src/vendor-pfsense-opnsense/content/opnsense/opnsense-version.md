---
id = "cmd/opnsense/opnsense-version"
kind = "command"
title = "opnsense-version"
category = "reference/vendors/pfsense-opnsense"
tags = ["opnsense", "cli", "system"]
risk = "info"
version = "OPNsense 24.x"
---
Prints the OPNsense product, version and build information.

## Command

```bash
opnsense-version
```

## Notes

- OPNsense equivalent of pfSense's `cat /etc/version`.
- Check the release before installing plugins or running upgrades.
- Combine with `cat /usr/local/etc/pkg/repos/OPNsense.conf` checks when a repo problem is suspected.

Source: OPNsense Documentation — https://docs.opnsense.org
