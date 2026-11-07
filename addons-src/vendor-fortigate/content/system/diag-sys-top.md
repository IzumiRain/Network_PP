---
id = "cmd/fortigate/diag-sys-top"
kind = "command"
title = "diagnose sys top"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "performance"]
risk = "info"
version = "FortiOS 7.4"
---
Live CPU and memory view per process, refreshed continuously until you stop it.

## Command

```cisco
diagnose sys top 5 10
```

## Notes

- Arguments are refresh interval and refresh count: this samples every 5 seconds for 10 rounds.
- The header shows load average and the CPU split (us/sy/wa/hi) plus memory usage.
- Look for a daemon pinned at ~100% CPU (for example ipsupd or scanupd) when the unit feels slow.
- Stop output with Ctrl+C; run it over SSH, not the console, on busy units.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
