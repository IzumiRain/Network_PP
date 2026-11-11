---
id = "doc/security/waf/modsecurity"
kind = "configuration"
title = "ModSecurity"
category = "reference/security/waf/modsecurity"
tags = ["waf", "concepts", "deployment", "security-model", "anomaly-scoring", "signatures", "virtual-patching", "modsecurity", "logging", "audit-log", "directives", "phases", "transformations", "sqli", "rules", "xss", "crs", "tuning", "false-positives", "model-doc"]
risk = "info"
version = "ModSecurity 2.9 / v3"
---
The complete ModSecurity document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Concepts
<!-- section-tags: waf, concepts, deployment, security-model, anomaly-scoring, signatures, virtual-patching -->


### Deployment modes: reverse proxy vs embedded

A WAF runs either as a standalone reverse-proxy tier in front of the app, or embedded inside the web server that serves the app.

#### Comparison

| Mode | Placement | Trade-off |
|---|---|---|
| Reverse proxy | Own server/VM/container in front of app servers | Clean scaling and TLS handling; extra hop and latency |
| Embedded | Module/connector in the app web server | No extra hop; consumes app server resources |

#### Notes

- Reverse proxy deployments decouple WAF upgrades from application releases and shield topology and TLS from the internet.
- Embedded (Apache module or nginx connector) is the classic ModSecurity setup and sees the same traffic the app sees.
- With libmodsecurity v3 the WAF logic is a library and the deployment shape depends on the connector used.
- X-Forwarded-For handling and header normalization must be correct, or both modes log/block on wrong client IPs.

Source: OWASP WAF / AppSec documentation — https://owasp.org


### Positive vs negative security model

A WAF either blocks what looks bad (negative security) or only allows what is known good (positive security).

#### Comparison

| Model | Logic | Strength | Weakness |
|---|---|---|---|
| Negative | Block signatures of known attacks | Quick to deploy, few breakages | Blind to novel attacks |
| Positive | Allow only expected methods, parameters and formats | Blocks unknown attacks too | Heavy per-app tuning effort |

#### Notes

- Signature-driven rule sets like OWASP CRS are negative-model with anomaly scoring on top.
- Positive rules are written per endpoint: allowed HTTP method, parameter names, value types and lengths.
- Production deployments usually layer both: positive rules on critical endpoints, CRS as the negative baseline.
- Start with the negative model in DetectionOnly, then add positive rules where the risk justifies the tuning cost.

Source: OWASP WAF / AppSec documentation — https://owasp.org


### Signature matching vs anomaly scoring

Two ways to reach a verdict: each rule blocks on its own (signature) or rules only add to a score that is judged at the end (anomaly scoring).

#### Notes

- Signature mode: one matching rule = immediate block; simple but a single over-broad rule disrupts users.
- Anomaly scoring: matching rules increment inbound/outbound scores; a final evaluation rule decides at the threshold.
- CRS 3.x and 4.x default to anomaly scoring, which gives one central decision point and better visibility per transaction.
- Score contributors are visible in the audit trailer, which makes exclusion work much more predictable than per-rule blocking.
- Blocking thresholds live in crs-setup.conf (inbound 5 / outbound 4 by default), separate from the individual rules.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


### Virtual patching

Deploying a WAF rule that blocks exploitation of a specific vulnerability before the application itself is fixed.

#### Example

```apache
# Temporary virtual patch while the app team fixes CVE-2024-0001
SecRule REQUEST_URI "@rx ^/api/legacy/transfer" \
  "id:900100,phase:1,deny,log,msg:'Virtual patch: CVE-2024-0001 endpoint blocked'"
```

#### Notes

- Buys time between disclosure and the real code fix; it is a mitigation, never a replacement for patching.
- Scope the rule as tightly as possible (URI, parameter, source) or it becomes a permanent source of false positives.
- Tag virtual patches (e.g. `tag:'vpatch-cve-2024-0001'`) and track expiry so they get removed after the fix ships.
- The OWASP CRS project and many vendors publish pre-made virtual-patch rule packs for active CVEs.

Source: OWASP WAF / AppSec documentation — https://owasp.org


## Logging
<!-- section-tags: waf, modsecurity, logging, audit-log -->


### Audit logging and log parts

ModSecurity audit logs record full transactions; log parts select which sections are written.

#### Example

```apache
SecAuditEngine RelevantOnly
SecAuditLogType Serial
SecAuditLog /var/log/modsec_audit.log
SecAuditLogParts ABFHZ
```

#### Notes

- `SecAuditEngine` is Off / On / RelevantOnly (only transactions that matched a rule - the usual production choice).
- Parts: A audit header, B request headers, C request body, E response body, F response headers, H trailer (rule messages), Z end boundary. Default is `ABCFHZ`.
- `SecAuditLogType Serial` writes one file; `Concurrent` writes per-transaction files - required for high volume and for tools that index single events.
- Part H is what tuning work reads: matched rule IDs, messages and anomaly score summary.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org


## Modsecurity
<!-- section-tags: waf, modsecurity, directives, phases, transformations -->


### SecRule syntax basics

Every CRS and custom rule is a SecRule: variables, an operator, and an action list.

#### Example

```apache
SecRule REQUEST_URI "@rx /admin" \
  "id:1002,phase:1,deny,log,status:403,msg:'Admin path touched'"
```

#### Notes

- Structure: `SecRule <variables> "<operator>" "<actions>"` - e.g. variables like `ARGS`, `REQUEST_HEADERS`, `REQUEST_URI`.
- `id` is mandatory and must be unique; CRS uses 9xxxxx ranges, pick a private range (1xxxxxx) for custom rules.
- `phase:1|2` targets request headers/body, `phase:3|4` response, `phase:5` logging.
- Disruptive actions (`deny`, `status`, `allow`) only take effect when SecRuleEngine is On; `log`/`nolog` and `msg`/`tag` shape the audit trail.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org


### SecRuleEngine directive

Master switch deciding whether rules execute disruptively, log only, or not at all.

#### Example

```apache
SecRuleEngine On        # On = enforce, DetectionOnly = match+log only, Off = disabled
```

#### Notes

- `On` is the enforcing mode; `DetectionOnly` runs the full rule set but never disrupts - ideal for the first weeks.
- `Off` disables rule processing entirely; audit logging of matched rules also stops.
- Deploy new rule sets in DetectionOnly, review matched rules in the audit log, then switch to `On` deliberately.
- The setting is global; per-transaction overrides use `ctl:ruleEngine` actions, not a second SecRuleEngine.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org


### ModSecurity transaction phases

Each request is processed through five phases; rule phase decides what data is available.

#### Phases

| Phase | Stage | Typical use |
|---|---|---|
| 1 | Request headers | URI/method/header checks, early deny |
| 2 | Request body | ARGS inspection, SQLi/XSS detection |
| 3 | Response headers | Outbound header checks |
| 4 | Response body | Outbound leakage/defacement checks |
| 5 | Logging | Custom audit and scoring logic |

#### Notes

- Body inspection requires request body access enabled: `SecRequestBodyAccess On` (and `SecResponseBodyAccess On` for phases 3-4).
- A deny in phase 1 or 2 stops the transaction before the backend sees it; response phases can only log or block the response.
- Rules using variables that do not exist in their phase simply never match - a common cause of "my rule never fires".

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org


### Transformation functions (t: actions)

Transformations normalize variable values before the operator runs, closing encoding bypasses.

#### Example

```apache
SecRule ARGS "@rx select" \
  "id:1003,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,msg:'SQLi keyword'"
```

#### Notes

- Common functions: `t:lowercase`, `t:urlDecodeUni`, `t:htmlEntityDecode`, `t:removeNulls`, `t:replaceComments`.
- Transformations chain in written order and the operator sees only the final transformed value.
- `t:none` clears transformations inherited from earlier rules in the chain.
- CRS rules list several transformations per rule; when writing exclusions or custom rules, mirror the same chain or detection can be bypassed by encoding.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org


## Rules
<!-- section-tags: waf, modsecurity, sqli, rules, xss -->


### SQL injection detection rule example

A minimal SQLi detection rule using libinjection, the same engine family CRS production rules build on.

#### Example

```apache
SecRule ARGS|ARGS_NAMES "@detectSQLi" \
  "id:1004,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,tag:'custom-sqli',msg:'SQL injection attempt'"
```

#### Notes

- `@detectSQLi` fingerprints SQLi payloads via libinjection; `@detectXSS` is its XSS counterpart.
- The CRS production equivalent lives in the 942-SQLI files (e.g. rule 942100) with many layered regexes and transformations.
- Keep the transformation chain (lowercase, urlDecodeUni) or encoded payloads pass through undetected.
- Under anomaly scoring, replace `deny` with a score increment and let the threshold rules make the final decision.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


### XSS detection rule example

A minimal XSS detection rule over arguments and referer header using libinjection.

#### Example

```apache
SecRule ARGS|REQUEST_HEADERS:Referer "@detectXSS" \
  "id:1005,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,tag:'custom-xss',msg:'XSS attempt'"
```

#### Notes

- `@detectXSS` recognizes common script/HTML injection fingerprints in any targeted variable.
- The CRS production equivalents are the 941-XSS rules (941100 onward), which add regex layers and HTML-entity handling.
- Response-body XSS is not covered by request-side rules; CRS outbound files (950s) handle response leakage.
- In DetectionOnly deployments this rule shape is the safest first test for request-body parsing.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org


## Tuning
<!-- section-tags: waf, crs, tuning, false-positives -->


### False-positive tuning: rule exclusions

Exclude specific rules or rule targets for trusted endpoints instead of disabling rules globally.

#### Example

```apache
# REQUEST-900-EXCLUSION-RULES-BEFORE-CRS.conf
# Remove one target (parameter) from one rule for one URI
SecRule REQUEST_URI "@beginsWith /api/search" \
  "id:1006,phase:1,pass,nolog,ctl:ruleRemoveTargetById=942100;ARGS:q"
# Global per-rule options
SecRuleRemoveById 920320
SecRuleUpdateTargetById 942100 "!ARGS:legacy_query"
```

#### Notes

- Narrowest first: target-scoped removal (rule + variable, ideally URI-scoped) beats dropping a whole rule.
- The REQUEST-900 exclusion file is the supported place for CRS exclusions and ships with documented templates.
- `SecRuleRemoveById` disables a rule entirely - audit log the reason and review it periodically.
- Repeat offenders at higher paranoia levels usually need their own exclusion set, not a lower PL globally.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org

