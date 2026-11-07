---
id = "cmd/paloalto/debug-dataplane-packet-diag"
kind = "command"
title = "debug dataplane packet-diag (packet capture)"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "capture", "troubleshooting"]
risk = "medium"
version = "PAN-OS 10.x/11.x"
---
Captures packets on the dataplane using a packet-diag filter and capture toggle.

## Command

```text
debug dataplane packet-diag set filter match source 10.1.1.10 destination 8.8.8.8
debug dataplane packet-diag set capture setting file cap1
debug dataplane packet-diag set capture on
debug dataplane packet-diag set capture off
```

## Notes

- Set the filter first so you capture only the flow you need, then enable capture.
- Capture files land in /var/log/pan/pcap/ - pull them off with SCP from the management interface.
- Always clean up afterwards: `set capture off`, and clear the filter (`set filter match source 0`).
- Capture can impact dataplane performance - keep captures short and tightly filtered.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
