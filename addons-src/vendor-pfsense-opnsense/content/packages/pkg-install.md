---
id = "cmd/pfsense/pkg-install"
kind = "command"
title = "pkg / pkg-static (install packages)"
category = "reference/vendors/pfsense-opnsense"
tags = ["pfsense", "opnsense", "freebsd", "cli", "packages"]
risk = "medium"
version = "pfSense CE 2.7 / OPNsense 24.x"
---
Installs FreeBSD packages via pkg; both distributions ship the vendor repo preconfigured.

## Command

```bash
pkg install <package-name>
pkg info <package-name>
```

## Notes

- pfSense scripts typically call the same binary as `pkg-static`; `pkg` works in the shell.
- Prefer the WebGUI package manager (System > Package Manager / Firmware > Plugins) for firewall packages.
- Manually installed packages may be removed on upgrade and are not covered by vendor support.
- `pkg info` lists what is installed before you change anything.

Source: Netgate pfSense Documentation — https://docs.netgate.com
