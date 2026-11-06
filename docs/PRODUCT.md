# Network++ — Product Definition

**Offline-first IT Field Toolkit**

A comprehensive toolkit for IT, network, OS, security and infrastructure
professionals: technical knowledge, practical tools, diagnostics and
operational guides — usable without an internet connection.

This document is the product contract. The IA below is the north star;
content fills it phase by phase (see the roadmap in README.md).

## Information architecture

Five primary sections plus Settings. The full taxonomy lives in
[PRODUCT-STRUCTURE-draft.txt](PRODUCT-STRUCTURE-draft.txt); the implemented
skeleton in 0.2:

```
Home          quick tools · pinned · recent · favorites · library & platform status
Reference     knowledge base built from content packs:
              os/linux · os/windows · monitoring/splunk · networking (fundamentals,
              addressing, switching, transport, services, security) · errors (http,
              network, tls) — grows with packs
Tools         calculators (CIDR/IP, ports) · packet lab (build/parse) — pure local
Diagnostics   host discovery & port scan · local network inspection · live connections
Runbooks      step-by-step operational guides with checklists and decisions
Settings      content packs (install/registry/proxy) · data & storage · about
```

Global capabilities (not navigation items): global search (Ctrl+K),
copy-to-clipboard everywhere, favorites, pin-to-home, related content
(via IDs), risk classification on executable commands, content versioning,
offline indicator, keyboard shortcuts (desktop).

## Content model

Nine entity templates. Every piece of content in every pack is one of these:

| Kind | Fields that matter |
|---|---|
| `command` | command, description, params, example, output, **risk** |
| `protocol` | purpose, structure, headers, fields, ports, RFC |
| `vendor` | products, OS families, versions, CLI commands |
| `tool` | description, input, output, limits, example |
| `runbook` | prerequisites, steps, decisions, validation, rollback |
| `error` | message, cause, diagnosis, remedy |
| `port` | number, protocol, service, usage, security notes |
| `configuration` | description, sample, version, restore path |
| `standard` | identifier, version, scope, official source |

Format: Markdown + TOML frontmatter (`id`, `kind`, `title`, `category`,
`tags`, `risk`, `version`). IDs are immutable slugs
(`cmd/linux/ip-a`, `err/http-status-codes`, `rb/net/no-connectivity`) and are
the single source of truth — categories and packs *reference* IDs, never copy
content.

## Content packs

Base app stays small; specialized knowledge ships as downloadable packs.

- Format: ZIP with `manifest.json` (schema 1: `id`, `title`, `version`,
  `min_app_version`) and `content/**/*.md`.
- Source: a GitHub repository holding `index.json` (registry: id, version,
  size, **sha256**, url) plus the pack zips. Any repo works — the registry URL
  is a setting.
- Mirrors for restricted networks: put the registry behind jsDelivr
  (`https://cdn.jsdelivr.net/gh/<user>/<repo>@main/index.json`); the app also
  supports a manual SOCKS5/HTTP proxy.
- Install paths: registry download (SHA-256 verified) or offline file import.
- Packs are **data only** — Markdown is rendered, never executed.
- The bundled `core` pack installs on first run and updates with the app.

## Architecture rules (fixed since 0.2)

1. **Content is never duplicated** — one ID, many entry points.
2. **Reference vs Runbook** — Reference says what something *is*; a Runbook
   says what to *do*, in what order, with validation and rollback.
3. **Platform differences live in code**, not content — a Linux guide renders
   identically on Android; the local-interface tool adapts per platform.
4. **Specialized content becomes packs**, keeping the core small.
5. **Operational content carries a source and version** — stale commands in
   production are a liability.

## Platform matrix

| Capability | Windows / Linux / macOS | Android |
|---|---|---|
| Reference, Runbooks, search, packs | ✓ | ✓ |
| Calculators, converters, packet lab | ✓ | ✓ |
| Scanning, local devices, connections | ✓ | ✗ (no raw sockets without root) |
| Raw packet capture (0.2+, `capture` feature) | ✓ (Npcap/libpcap present) | ✗ |
