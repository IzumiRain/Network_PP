---
id = "ref/waf/signature-vs-anomaly-scoring"
kind = "protocol"
title = "Signature matching vs anomaly scoring"
category = "reference/security/waf/concepts"
tags = ["waf", "concepts", "anomaly-scoring", "signatures"]
version = "OWASP CRS 3.x/4.x"
---
Two ways to reach a verdict: each rule blocks on its own (signature) or rules only add to a score that is judged at the end (anomaly scoring).

## Notes

- Signature mode: one matching rule = immediate block; simple but a single over-broad rule disrupts users.
- Anomaly scoring: matching rules increment inbound/outbound scores; a final evaluation rule decides at the threshold.
- CRS 3.x and 4.x default to anomaly scoring, which gives one central decision point and better visibility per transaction.
- Score contributors are visible in the audit trailer, which makes exclusion work much more predictable than per-rule blocking.
- Blocking thresholds live in crs-setup.conf (inbound 5 / outbound 4 by default), separate from the individual rules.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
