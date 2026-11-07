---
id = "cmd/asa/show-cpu-usage"
kind = "command"
title = "show cpu usage"
category = "reference/firewalls/cisco-asa/system"
tags = ["asa", "cisco", "cli", "system", "performance"]
risk = "info"
version = "ASA 9.16"
---
Shows CPU utilization over 5-second, 60-second and 5-minute intervals.

## Command

```cisco
show cpu usage
```

## Notes

- Sustained high CPU is usually check-inspection load, logging volume or a traffic anomaly.
- Compare the 5-second vs 5-minute values: a spike pattern points to bursts, a flat 100% to overload.
- Follow up with `show processes cpu-hog` to find processes that held the CPU for long stretches.
- Multi-core platforms show a total across cores; per-core detail is available in ASDM/monitoring.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
