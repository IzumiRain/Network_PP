---
id = "ref/fortiweb/waf-signature-syntax"
kind = "configuration"
title = "Custom WAF signature syntax basics"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "signatures"]
version = "FortiWeb 7.2"
---
Custom signatures match a request attribute against a regular expression and assign a severity and action; the exact grammar is defined in the FortiWeb Administration Guide custom signature chapter.

## Building blocks

- Match target: a request element such as URI, headers, body or method.
- Pattern: a regular expression evaluated against that target.
- Action: alert, block or similar, plus a severity used for reporting and exceptions.

## Notes

- Write signatures narrow: a broad regex blocks legitimate traffic and floods the logs.
- Test new signatures in alert-only action first, then switch to block once the baseline is clean.
- Attach signatures via signature groups in the web protection profile, not per server policy directly.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
