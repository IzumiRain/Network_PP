---
id = "ref/fortiweb/ml-protection"
kind = "configuration"
title = "ML-based protection"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "machine-learning"]
version = "FortiWeb 7.2"
---
Machine-learning based protection builds a model of normal traffic for a site (parameters, URL structure, file uploads) and flags requests that deviate from it.

## How it works

- Learning mode records legitimate traffic; the model matures over days of representative traffic.
- After the baseline stabilizes, the policy switches to detecting anomalies against that model.
- Works where signature-based rules struggle: application-specific parameters and encoded payloads.

## Notes

- Run learning only on trusted traffic windows; attackers in learning mode poison the baseline.
- Review anomaly reports and tune before switching to enforcement to avoid false positives.
- ML complements signatures; keep signature groups enabled in the same protection profile.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
