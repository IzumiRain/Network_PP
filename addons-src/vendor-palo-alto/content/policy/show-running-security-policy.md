---
id = "cmd/paloalto/show-running-security-policy"
kind = "command"
title = "show running security-policy"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "policy", "security"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Shows the security policy actually compiled and loaded into the running configuration.

## Command

```text
show running security-policy
```

## Notes

- Reflects the policy the dataplane enforces, including rules pushed by Panorama.
- Ends with the implicit `intrazone-default` and `interzone-default` rules.
- Compare against the candidate policy in the GUI when traffic does not behave after a commit.
- Use with `show session all` to see which running rule a flow matched.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
