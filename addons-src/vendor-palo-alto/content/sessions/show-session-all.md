---
id = "cmd/paloalto/show-session-all"
kind = "command"
title = "show session all"
category = "reference/firewalls/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "sessions", "troubleshooting"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Lists active sessions with source, destination, application and the matching rule.

## Command

```text
show session all
show session all filter source 10.1.1.10 destination 8.8.8.8
show session id 1234
```

## Notes

- The filter form narrows output to the pair you care about - run `show session count` first on busy firewalls.
- `show session id <n>` gives full detail including NAT, QoS and packet counters.
- Check which security rule and egress interface a flow actually hit.
- Missing sessions mean the traffic was dropped before session setup or is not arriving.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
