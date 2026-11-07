---
id = "cmd/fortiweb/show-waf-dos-protection"
kind = "command"
title = "show waf dos-protection"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "cli", "dos"]
risk = "info"
version = "FortiWeb 7.2"
---
Prints the DoS protection policy: SYN flood, HTTP flood and related thresholds plus their actions.

## Command

```cisco
show waf dos-protection
```

## Notes

- Thresholds too low block legitimate bursts (launches, sales events); too high and floods pass through.
- HTTP flood thresholds are per-request-rate; pair with rate limiting and block periods for repeat offenders.
- The policy only applies where it is referenced by the protection profile on a server policy.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
