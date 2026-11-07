---
id = "cmd/paloalto/show-jobs-all"
kind = "command"
title = "show jobs all"
category = "reference/firewalls/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "jobs", "commit"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Lists commit, install and upgrade jobs with ID, status and progress.

## Command

```text
show jobs all
show jobs id 25
```

## Notes

- Confirm a commit or software install actually finished here before further changes.
- `show jobs id <n>` shows per-job detail including errors and the triggering user.
- Jobs stuck at a low percentage for a long time usually mean a pending reboot or hung process.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
