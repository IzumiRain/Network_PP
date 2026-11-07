---
id = "cmd/checkpoint/fw-monitor"
kind = "command"
title = "fw monitor (packet capture)"
category = "reference/vendors/check-point"
tags = ["checkpoint", "cli", "capture", "troubleshooting"]
risk = "low"
version = "Gaia R81.x"
---
Captures packets as they pass the firewall inspection points.

## Command

```text
fw monitor -e 'accept src=10.0.0.5 and dst=8.8.8.8;'
```

## Notes

- Runs from Expert mode; shows the same packet at each inspection point (i/I/o/O).
- Ctrl+C stops the capture and prints the per-point summary.
- Add `-o <file.cap>` to write a pcap for Wireshark instead of the console.
- Keep the accept filter tight - a wide filter floods the console and skews results.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
