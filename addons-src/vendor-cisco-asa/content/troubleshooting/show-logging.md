---
id = "cmd/asa/show-logging"
kind = "command"
title = "show logging"
category = "reference/firewalls/cisco-asa/troubleshooting"
tags = ["asa", "cisco", "cli", "logging"]
risk = "info"
version = "ASA 9.16"
---
Shows logging configuration state, enabled destinations and the buffered log tail.

## Command

```cisco
show logging
logging buffered informational
logging host inside 10.9.9.9
```

## Notes

- Confirms whether logging is enabled and at what severity each destination (buffered, syslog server, ASDM) is set.
- The buffered log prints inline below the config summary - filter with `show logging | include 106023` for deny hits.
- Message 106023 (ACL deny) and 302013/302014 (flow build/teardown) are the everyday troubleshooting messages.
- Use `logging list <name> <level|class>` to define what a syslog destination receives; avoid debug levels in production.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
