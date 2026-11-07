---
id = "cmd/fortigate/execute-log-display"
kind = "command"
title = "execute log display"
category = "reference/vendors/fortinet/fortigate/troubleshooting"
tags = ["fortigate", "fortios", "cli", "logging"]
risk = "info"
version = "FortiOS 7.4"
---
Prints locally stored logs matching the current execute log filter criteria directly to the CLI.

## Command

```cisco
execute log display
```

## Notes

- Quick triage path when the GUI is unreachable or you already have an SSH session open.
- Large result sets paginate slowly; narrow the filter (category, free-text, time range) first.
- Requires disk logging to be enabled; for full retention and reporting use FortiAnalyzer.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
