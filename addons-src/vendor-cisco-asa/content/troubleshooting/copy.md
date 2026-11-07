---
id = "cmd/asa/copy"
kind = "command"
title = "copy (file management: TFTP/SCP/HTTP)"
category = "reference/vendors/cisco/asa/troubleshooting"
tags = ["asa", "cisco", "cli", "files", "config"]
risk = "medium"
version = "ASA 9.16"
---
Copies files to/from flash - software images, ASDM images, configs and capture exports.

## Command

```cisco
copy tftp://10.9.9.9/asa9-16-3-19.bin flash:/asa9-16-3-19.bin
copy run tftp://10.9.9.9/asa-config.cfg
boot system flash:/asa9-16-3-19.bin
```

## Notes

- Copying a file over the current boot image is destructive to that file - always copy to a new name, `dir` to verify, then set `boot system`.
- After changing the boot variable, save with `write memory` so the change survives reload; confirm with `show version` (boot image line).
- Config backups: `copy running-config tftp://...`; restores are done by editing or `configure terminal` merge, not a blind overwrite.
- Both source and destination are URL-style; use `copy /pcap capture:CAP1 ...` for capture exports.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
