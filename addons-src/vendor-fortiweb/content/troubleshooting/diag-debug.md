---
id = "cmd/fortiweb/diag-debug"
kind = "command"
title = "diagnose debug and packet capture"
category = "reference/firewalls/fortiweb/troubleshooting"
tags = ["fortiweb", "waf", "cli", "debug"]
risk = "info"
version = "FortiWeb 7.2"
---
Enables CLI debug output and captures packets on an interface to verify what traffic actually reaches the appliance.

## Command

```cisco
diagnose debug enable
diagnose sniffer packet port1 "tcp port 443" 4
diagnose debug disable
```

## Notes

- diagnose debug enable turns on output for subsequent debug commands; disable it immediately after use.
- The sniffer arguments are the interface, a tcpdump-style filter in quotes, and the number of packets to capture.
- Use it to confirm requests arrive on the expected interface/port before blaming WAF rules for failures.
- The GUI also offers packet capture and detailed log views when CLI access is limited.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
