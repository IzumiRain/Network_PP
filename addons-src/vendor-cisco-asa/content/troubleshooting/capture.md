---
id = "cmd/asa/capture"
kind = "command"
title = "capture / show capture (inline packet capture)"
category = "reference/vendors/cisco/asa/troubleshooting"
tags = ["asa", "cisco", "cli", "troubleshooting", "capture"]
risk = "medium"
version = "ASA 9.16"
---
Captures packets on a named interface with an ACL-style match filter; remove captures when done.

## Command

```cisco
capture CAP1 interface inside match ip host 10.1.1.1 host 8.8.8.8
show capture CAP1
show capture CAP1 detail
copy /pcap capture:CAP1 tftp://10.9.9.9/cap1.pcap
no capture CAP1
```

## Notes

- The capture definition lives in the running config (shows in `show run all capture`), so always `no capture` after the session.
- Default buffers are small; use `buffer 2000000` and `packet-length`/`circular-buffer` options for longer sessions.
- Browse `https://<asa>/admin/capture/CAP1` to download or inspect a capture without CLI access.
- Heavy captures can impact CPU; scope with `match` or an access-list instead of capturing an entire interface.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
