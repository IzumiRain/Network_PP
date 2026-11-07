---
id = "cmd/fortiweb/execute-backup-config"
kind = "command"
title = "execute backup config"
category = "reference/firewalls/fortiweb/system"
tags = ["fortiweb", "waf", "cli", "backup"]
risk = "low"
version = "FortiWeb 7.2"
---
Pushes the FortiWeb configuration to an external server (TFTP shown; FTP/SFTP variants are available).

## Command

```cisco
execute backup config tftp fwb-backup.conf 10.0.0.50
```

## Notes

- First argument is the file name on the server, second is the server address.
- Take a backup before signature policy changes or firmware upgrades.
- The file includes WAF policies and protection profiles, so treat it as sensitive.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
