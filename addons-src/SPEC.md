# Pack authoring instructions (for content contributors)

Write content packs for Network++. Every pack is a folder:

```
addons-src/<pack-id>/
├── manifest.json
└── content/
    └── ...folders.../*.md
```

## manifest.json (exact shape)

```json
{
  "schema": 1,
  "id": "vendor-fortigate",
  "title": "Fortinet FortiGate",
  "version": "1.0.0",
  "min_app_version": "0.2.0",
  "description": "One line.",
  "author": "Network++ content team"
}
```

`id`: lowercase letters/digits/dashes only. `version`: three-part semver.

## Entity file (Markdown + TOML frontmatter)

Every `content/**/*.md` file is ONE entity. Frontmatter is TOML:

```markdown
---
id = "cmd/fortigate/show-system-global"
kind = "command"
title = "show system global"
category = "reference/firewalls/fortigate"
tags = ["fortigate", "fortios", "cli"]
risk = "info"
version = "FortiOS 7.4"
---
One short intro sentence.

## Command

```cisco
show system global
```

## Notes

- what it shows / when to use it

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
```

### Rules (non-negotiable)

1. `id` — lowercase `a-z 0-9 / - _`, unique, prefixed by pack domain
   (`cmd/fortigate/...`, `err/fortiweb/...`, `std/iso27001`).
2. `kind` — one of: `command`, `protocol`, `vendor`, `tool`, `runbook`,
   `error`, `port`, `configuration`, `standard`.
3. `category` — slug path: firewalls go under `reference/firewalls/<vendor>`,
   network gear under `reference/devices/<vendor>`, monitoring under
   `reference/monitoring/<tool>`, security knowledge under
   `reference/security/...`, errors under `reference/errors/...`,
   runbooks under `runbooks/...`.
4. `risk` — REQUIRED for every `command`: `info` (read-only display),
   `low` (changes reporting/cache), `medium` (changes config state),
   `high` (destructive / service-affecting). When in doubt, rate higher.
5. `version` — the OS/product version the command applies to.
6. **Only include commands you are certain about.** Every entity body ends
   with `Source: <product> <guide name> — https://<official-docs-domain>`
   (vendor's own documentation domain, e.g. docs.fortinet.com,
   cisco.com, techdocs.f5.com, help.splunk.com). Never invent deep URLs —
   cite the product/guide name plus the docs domain root or a stable
   section page.
7. Body: short intro sentence, the command/syntax in a fenced code block,
   2-6 bullet notes (what it shows, when to use, gotchas). Compact —
   8-25 lines per entity.
8. No HTML in bodies; Markdown only (tables, fenced code, task lists OK).
9. 12-25 entities per pack. Prefer breadth of daily-use commands over
   depth of one topic.
10. TOML strings with quotes/backslashes must be escaped (`\"`).

## Pack set for this release

| Pack id | Title | Domain | Category root |
|---|---|---|---|
| vendor-fortigate | Fortinet FortiGate | FortiOS CLI | reference/firewalls/fortigate |
| vendor-fortiweb | Fortinet FortiWeb | WAF | reference/firewalls/fortiweb |
| vendor-fortiauthenticator | Fortinet FortiAuthenticator | identity | reference/security/fortiauthenticator |
| vendor-cisco-asa | Cisco Secure Firewall (ASA) | ASA/FTD CLI | reference/firewalls/cisco-asa |
| vendor-f5-bigip | F5 BIG-IP | tmsh | reference/devices/f5-bigip |
| vendor-waf | Web Application Firewalls (ModSecurity & concepts) | WAF | reference/security/waf |
| splunk-operations | Splunk operations | deployment/troubleshooting | reference/monitoring/splunk-ops |
| security-plus | Security+ core, signature types, OWASP Top 10 | knowledge | reference/security/security-plus |
| iso-standards | ISO/IEC standards (27001 family and related) | standards | reference/security/iso-standards |
| vendor-juniper | Juniper Junos | CLI | reference/devices/juniper |
| vendor-mikrotik | MikroTik RouterOS | CLI | reference/devices/mikrotik |
| vendor-palo-alto | Palo Alto PAN-OS | CLI | reference/firewalls/palo-alto |
| vendor-pfsense-opnsense | pfSense / OPNsense | CLI | reference/firewalls/pfsense-opnsense |
| vendor-checkpoint | Check Point Gaia | CLI | reference/firewalls/checkpoint |
| vendor-sophos | Sophos Firewall | CLI | reference/firewalls/sophos |
| vendor-sonicwall-watchguard | SonicWall & WatchGuard | CLI | reference/firewalls/sonicwall-watchguard |
| vendor-proxmox | Proxmox VE | qm/pct/CLI | reference/virtualization/proxmox |
| vendor-docker-k8s | Docker & Kubernetes | CLI | reference/virtualization/docker-k8s |
| cloud-aws-azure-gcp | AWS / Azure / GCP networking CLI | cloud | reference/cloud/aws-azure-gcp |
| monitoring-elk-prometheus | ELK stack, Prometheus, Grafana, Zabbix | monitoring | reference/monitoring/elk-prometheus |
