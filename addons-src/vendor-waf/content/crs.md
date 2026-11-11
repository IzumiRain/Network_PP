---
id = "doc/security/waf/crs"
kind = "protocol"
title = "OWASP ModSecurity Core Rule Set (CRS)"
category = "reference/security/waf/crs"
tags = ["waf", "crs", "anomaly-scoring", "tuning", "paranoia-level", "rule-ids", "model-doc"]
risk = "info"
version = "OWASP CRS 3.x/4.x"
---
The complete OWASP ModSecurity Core Rule Set (CRS) document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## CRS
<!-- section-tags: waf, crs, anomaly-scoring, tuning, paranoia-level, rule-ids -->


### Anomaly score thresholds

Inbound and outbound score thresholds decide when a transaction is blocked under anomaly scoring.

#### Example

```apache
# crs-setup.conf (defaults)
# setvar:tx.inbound_anomaly_score_threshold=5
# setvar:tx.outbound_anomaly_score_threshold=4
```

#### Notes

- Default thresholds are 5 (inbound) and 4 (outbound); each matching CRS rule adds its own score contribution.
- Raise thresholds for sites with unavoidable partial matches; lower them to block sooner at the cost of false positives.
- The final evaluation rules 949110 (inbound) and 959100 (outbound) fire at the threshold and produce the block decision.
- Thresholds work together with paranoia level: raising PL adds rules and their scores, which can raise effective strictness twice.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


### CRS modes: traditional blocking vs anomaly scoring

The CRS can block per matched rule (traditional mode) or accumulate scores and block at a threshold (anomaly scoring, the default since CRS 3.x).

#### Example

```apache
# crs-setup.conf - traditional mode is OFF by default; both lines uncomment it
#SecDefaultAction "phase:1,deny,log,status:403"
#SecDefaultAction "phase:2,deny,log,status:403"
```

#### Notes

- Keep the SecDefaultAction lines commented to stay in anomaly scoring mode (recommended and default).
- Traditional mode blocks on the first matching rule, which loses score detail and makes self-matching rules disrupt traffic.
- In anomaly scoring the final evaluation rules (949110 inbound, 959100 outbound) apply the configured thresholds.
- Never run CRS 3.x+ in traditional mode for a new deployment; older CRS 2.x setups are the main reason the mode still exists.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


### CRS paranoia levels (PL1-PL4)

Paranoia levels trade strictness against false positives: PL1 ships as default, PL4 enables the most aggressive rule subset.

#### Example

```apache
# crs-setup.conf
# setvar:tx.paranoia_level=1
```

#### Notes

- PL1: baseline for most sites; PL2-3 add stricter checks (detection of unusual encodings, tighter length/character rules).
- PL4 treats even slight deviations as attacks and will break many legitimate applications - test before raising.
- Every rule is tagged with its paranoia level; rules above the configured level are skipped at runtime.
- Raise the level gradually per site and pair each bump with a review of the audit log for new false positives.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


### CRS rule groups (rule file map)

CRS rules live in numbered files under the rules/ directory; rule IDs share their file's numeric prefix.

#### Key groups

| Prefix | Rule file | Purpose |
|---|---|---|
| 920 | 920-PROTOCOL-ENFORCEMENT | HTTP conformance (methods, headers, versions) |
| 921 | 921-PROTOCOL-ATTACK | Request smuggling, splitting, poisoning |
| 930-933 | LFI / RFI / RCE / PHP files | Path, injection and code-execution families |
| 941 | 941-XSS | Cross-site scripting |
| 942 | 942-SQLI | SQL injection |
| 943 | 943-SESSION-FIXATION | Session fixation |
| 944 | 944-RCE-JAVA | Java deserialization / Java RCE |

#### Notes

- When the audit log reports rule id 942xxxx, the tuning targets the 942-SQLI file - the prefix maps 1:1.
- Inbound files run in phases 1-2; response-side rules (959/950 files) evaluate outbound content.
- Load CRS by including the CRS setup then the rule files; individual files can be excluded from the include list if a family does not apply.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org

