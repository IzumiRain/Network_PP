# Network++ Content Pack Specification

Version **1** of the pack format (what app versions 0.2+ install).

A content pack is a ZIP archive holding Markdown knowledge. Packs are data
only — the app renders them, never executes them.

## Layout

```
your-pack.zip
├── manifest.json
└── content/
    └── ...any folder structure...
        └── *.md          ← every .md file is one entity
```

## manifest.json

```json
{
  "schema": 1,
  "id": "vendor-cisco",
  "title": "Cisco reference",
  "version": "1.0.0",
  "min_app_version": "0.2.0",
  "description": "IOS/IOS-XE/NX-OS commands and configuration guides.",
  "author": "you"
}
```

Rules: `schema` must be `1`; `id` is lowercase letters/digits/dashes only;
`version` is three-part semver; the app refuses packs whose
`min_app_version` is newer than itself.

## Entity files (Markdown + TOML frontmatter)

Every `content/**/*.md` is one entity. The file starts with TOML
frontmatter, then the body in Markdown (tables, fenced code, task lists all
render):

```markdown
---
id = "cmd/ios/show-interface-status"
kind = "command"
title = "show interfaces status"
category = "reference/devices/cisco/ios"
tags = ["cisco", "switching"]
risk = "info"
version = "IOS 15.x"
---
Shows port, name, status, vlan and duplex for every switchport.

```cisco
Switch# show interfaces status
```
```

### Field rules

| Field | Required | Rules |
|---|---|---|
| `id` | ✓ | immutable slug: `a-z 0-9 / - _`; never reused, never renamed |
| `kind` | ✓ | one of `command, protocol, vendor, tool, runbook, error, port, configuration, standard` |
| `title` | ✓ | human-readable |
| `category` | ✓ | slug path under a root section (`reference/…`, `runbooks/…`) |
| `tags` | – | list of strings |
| `risk` | – | `info`, `low`, `medium`, `high` — classify executable commands! |
| `version` | – | source/applicability version string |

Invalid files are skipped with a warning; a pack installs if at least one
entity parses.

## Distribution

### Registry (auto-download)

Host a repo with:

```
index.json          ← {"packs":[{id,title,version,description,size,sha256,url,min_app_version}]}
packs/vendor-cisco-1.0.0.zip
```

Point users to the raw URL — or the jsDelivr mirror of the same file:
`https://cdn.jsdelivr.net/gh/<user>/<repo>@main/index.json` (reachable from
networks where GitHub raw is blocked). Publish the SHA-256 of each zip in
the index; the app verifies it before installing.

### Offline import

Users can install any zip directly: Settings → Content packs → Install from
file.

## Publishing checklist

- [ ] every entity id unique and immutable
- [ ] commands carry `risk` and a `version` they apply to
- [ ] zip ≤ 100 MB, each file ≤ 10 MB, text-first (SVG only, no bitmaps)
- [ ] sha256 of the zip recorded in your index.json
