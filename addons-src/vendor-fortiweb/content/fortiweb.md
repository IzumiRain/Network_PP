---
id = "doc/fortinet/fortiweb/fortiweb"
kind = "vendor"
title = "Fortinet FortiWeb"
category = "reference/vendors/fortinet/fortiweb"
tags = ["fortiweb", "waf", "cli", "policies", "api", "machine-learning", "dos", "geo", "ip-reputation", "signatures", "protection", "ssl", "certificates", "backup", "debug", "model-doc"]
risk = "low"
version = "FortiWeb 7.2"
---
The complete Fortinet FortiWeb document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Policies
<!-- section-tags: fortiweb, waf, cli, policies -->


### show server-policy

Prints the server policies: which interfaces and ports accept client traffic, which server pools answer, and which protection profile applies.

#### Command

```cisco
show server-policy
```

#### Notes

- The server policy is the traffic entry point; a request that never matches one never reaches WAF inspection.
- Check the frontend type (HTTP vs HTTPS) and certificate binding when clients fail TLS.
- The http-policy or inline protection profile field links the policy to its WAF enforcement.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show server-pool

Lists server pools: the real web/application servers behind FortiWeb, their ports, health checks and load balancing method.

#### Command

```cisco
show server-pool
```

#### Notes

- Verify member IPs and ports first when FortiWeb returns 502/503 style errors.
- Health check configuration per member shows why a server is marked down.
- The balancing method (round robin, least connections) explains unequal traffic distribution.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


## Protection
<!-- section-tags: fortiweb, waf, api, machine-learning, cli, dos, geo, ip-reputation, signatures, protection, ssl, certificates -->


### API protection

API protection enforces a strict contract on JSON/XML traffic: only requests matching the declared API schema are forwarded, everything else is logged or blocked.

#### Key pieces

- Schema import: OpenAPI (Swagger) or manually declared paths, methods and parameter types.
- Validation: FortiWeb rejects unknown fields, wrong types and out-of-spec requests before they reach the backend.
- Rate limiting and authentication offload are typically layered on the same API rules.

#### Notes

- The schema must track the application: every legitimate API change needs a FortiWeb update or traffic gets blocked.
- Start in alert-only mode to measure drift between schema and real traffic.
- API rules bind through the protection profile like other WAF features.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### ML-based protection

Machine-learning based protection builds a model of normal traffic for a site (parameters, URL structure, file uploads) and flags requests that deviate from it.

#### How it works

- Learning mode records legitimate traffic; the model matures over days of representative traffic.
- After the baseline stabilizes, the policy switches to detecting anomalies against that model.
- Works where signature-based rules struggle: application-specific parameters and encoded payloads.

#### Notes

- Run learning only on trusted traffic windows; attackers in learning mode poison the baseline.
- Review anomaly reports and tune before switching to enforcement to avoid false positives.
- ML complements signatures; keep signature groups enabled in the same protection profile.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show waf dos-protection

Prints the DoS protection policy: SYN flood, HTTP flood and related thresholds plus their actions.

#### Command

```cisco
show waf dos-protection
```

#### Notes

- Thresholds too low block legitimate bursts (launches, sales events); too high and floods pass through.
- HTTP flood thresholds are per-request-rate; pair with rate limiting and block periods for repeat offenders.
- The policy only applies where it is referenced by the protection profile on a server policy.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show waf geo-ip-policy

Prints the geo-IP policies: country lists that are allowed or blocked for the protected sites.

#### Command

```cisco
show waf geo-ip-policy
```

#### Notes

- Typical pattern is allow-list mode for country-specific sites and block-list mode for global ones.
- Geo decisions run before most WAF inspection, so an overly wide block list cuts traffic early.
- Check the action (alert vs block) when users report being unable to reach the site from abroad.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show waf ip-reputation

Prints the IP reputation policies: which FortiGuard IP reputation categories are blocked and the configured action.

#### Command

```cisco
show waf ip-reputation
```

#### Notes

- Categories cover known attackers, spam sources, anonymizers (VPN/Tor) and similar lists.
- Blocking anonymizer categories can hurt legitimate users behind corporate VPNs — review before enforcing.
- The policy must be attached in the web protection profile to take effect.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show waf signature

Lists the custom WAF signatures defined on the appliance, with their match conditions and actions.

#### Command

```cisco
show waf signature
```

#### Notes

- Custom signatures live under config waf signature and are combined into signature groups for a profile.
- Use it to audit what site-specific rules exist before tuning or deleting anything.
- Built-in FortiGuard signatures are separate and updated by subscription; this shows only your own rules.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### show waf web-protection-profile

Prints the web protection profiles: the bundles of signature groups, exceptions, HTTP access rules and DoS settings applied to server policies.

#### Command

```cisco
show waf web-protection-profile
```

#### Notes

- The profile is the middle layer: signature groups and other WAF features are attached here, then to the server policy.
- Use it to see which detection features are actually enabled for a protected site.
- An empty or minimal profile means most WAF checks are not enforced.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### SSL/TLS termination and certificates

FortiWeb terminates TLS in front of the web servers: HTTPS server policies present a server certificate, inspect plaintext HTTP, and re-encrypt towards the pool if configured.

#### Key pieces

- Server certificate: imported (with key) or generated on the appliance, then bound to the HTTPS server policy.
- Trusted CAs: imported for validating upstream servers and, where used, client certificates.
- TLS settings: protocol versions and cipher suites per policy; older versions should be disabled.

#### Notes

- A server policy mismatch between SNI and certificate causes client trust errors; test with a browser and a TLS scanner.
- Certificate imports and renewals are managed in the GUI certificate store; keep expiry dates in monitoring.
- After replacing a certificate, re-check the server policy binding before decommissioning the old one.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### Custom WAF signature syntax basics

Custom signatures match a request attribute against a regular expression and assign a severity and action; the exact grammar is defined in the FortiWeb Administration Guide custom signature chapter.

#### Building blocks

- Match target: a request element such as URI, headers, body or method.
- Pattern: a regular expression evaluated against that target.
- Action: alert, block or similar, plus a severity used for reporting and exceptions.

#### Notes

- Write signatures narrow: a broad regex blocks legitimate traffic and floods the logs.
- Test new signatures in alert-only action first, then switch to block once the baseline is clean.
- Attach signatures via signature groups in the web protection profile, not per server policy directly.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


## System
<!-- section-tags: fortiweb, waf, cli, backup -->


### execute backup config

Pushes the FortiWeb configuration to an external server (TFTP shown; FTP/SFTP variants are available).

#### Command

```cisco
execute backup config tftp fwb-backup.conf 10.0.0.50
```

#### Notes

- First argument is the file name on the server, second is the server address.
- Take a backup before signature policy changes or firmware upgrades.
- The file includes WAF policies and protection profiles, so treat it as sensitive.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


### get system status

Shows the FortiWeb appliance summary: firmware version, model, serial number and uptime.

#### Command

```cisco
get system status
```

#### Notes

- First command on any CLI session; confirms firmware build before you touch WAF policies.
- Serial number and version are required when opening FortiCare cases.
- Check that signature updates are flowing after a network change (dashboard or update log).

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com


## Troubleshooting
<!-- section-tags: fortiweb, waf, cli, debug -->


### diagnose debug and packet capture

Enables CLI debug output and captures packets on an interface to verify what traffic actually reaches the appliance.

#### Command

```cisco
diagnose debug enable
diagnose sniffer packet port1 "tcp port 443" 4
diagnose debug disable
```

#### Notes

- diagnose debug enable turns on output for subsequent debug commands; disable it immediately after use.
- The sniffer arguments are the interface, a tcpdump-style filter in quotes, and the number of packets to capture.
- Use it to confirm requests arrive on the expected interface/port before blaming WAF rules for failures.
- The GUI also offers packet capture and detailed log views when CLI access is limited.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com

