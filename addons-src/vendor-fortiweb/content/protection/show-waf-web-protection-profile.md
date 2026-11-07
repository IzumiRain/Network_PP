---
id = "cmd/fortiweb/show-waf-web-protection-profile"
kind = "command"
title = "show waf web-protection-profile"
category = "reference/vendors/fortinet/fortiweb/protection"
tags = ["fortiweb", "waf", "cli", "protection"]
risk = "info"
version = "FortiWeb 7.2"
---
Prints the web protection profiles: the bundles of signature groups, exceptions, HTTP access rules and DoS settings applied to server policies.

## Command

```cisco
show waf web-protection-profile
```

## Notes

- The profile is the middle layer: signature groups and other WAF features are attached here, then to the server policy.
- Use it to see which detection features are actually enabled for a protected site.
- An empty or minimal profile means most WAF checks are not enforced.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
