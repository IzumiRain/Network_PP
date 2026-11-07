---
id = "cmd/fortigate/execute-backup-config"
kind = "command"
title = "execute backup config"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "backup"]
risk = "low"
version = "FortiOS 7.4"
---
Pushes a copy of the running configuration to an external server (TFTP shown; FTP variant is available).

## Command

```cisco
execute backup config tftp fgt-backup.conf 10.0.0.50
```

## Notes

- The first argument is the file name on the TFTP server, the second the server IP.
- Take an off-box backup before every upgrade or risky change; it is the fastest rollback artifact.
- The file contains the full config including encrypted secrets, so store it on a trusted server.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
