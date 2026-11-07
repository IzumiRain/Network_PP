---
id = "cmd/f5-bigip/show-sys-version"
kind = "command"
title = "show sys version"
category = "reference/vendors/f5/bigip/system"
tags = ["f5", "bigip", "tmsh"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows the running BIG-IP software version, build, edition and platform model.

## Command

```bash
tmsh show sys version
```

## Notes

- First command on any tmsh session; confirm the version/build before applying known-issue fixes.
- The Edition line distinguishes point releases that carry specific bugfixes.
- On HA pairs run it on both units — version drift between peers blocks config sync.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
