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

### Pack-level categories (optional)

A manifest may declare `"categories": ["reference/vendors/fortinet/fortigate", ...]`
— the taxonomy roots the pack contributes to. Vendor product packs live
**vendor-first**: `reference/vendors/<company>/<product>/...`
(e.g. all Fortinet products under `vendors/fortinet/`). Pure knowledge packs
(study guides, standards, error references) live under their domain root
(`reference/security/...`, `reference/errors/...`).

## Field rules

| Field | Required | Rules |
|---|---|---|
| `id` | ✓ | immutable slug: `a-z 0-9 / - _`; never reused, never renamed |
| `kind` | ✓ | one of `command, protocol, vendor, tool, runbook, error, port, configuration, standard` |
| `title` | ✓ | human-readable |
| `category` | ✓ | slug path; vendor products use `reference/vendors/<company>/<product>/…` |
| `tags` | – | list of strings |
| `risk` | – | `info`, `low`, `medium`, `high` — classify executable commands! |
| `version` | – | source/applicability version string |

Invalid files are skipped with a warning; a pack installs if at least one
entity parses.

## Document model (v2 draft, dev branch)

> Status: **draft on `dev/data-driven-model-docs`**. v1 (above) still ships;
> v2 changes how content is *organised*, not how packs are delivered — a pack
> is still a zip users install selectively from the addons registry.

### One document per Model/id

Instead of many small files (one per command), a vendor model gets **one**
Markdown document containing *all* of its documentation:

```
content/
  bigip-4000.md        ← F5 BIG-IP 4000: the whole model, one file
```

Frontmatter follows v1 field rules, with these conventions:

| Field | v2 rule |
|---|---|
| `id` | ends with the model/id slug: `doc/<company>/<product>/<model>` — e.g. `doc/f5/bigip/4000` |
| `kind` | usually `vendor` for model documents (runbook documents keep `runbook`) |
| `title` | the model name as users know it: `F5 BIG-IP 4000` |
| `category` | `reference/vendors/<company>/<product>/<model>` — e.g. `reference/vendors/f5/bigip/4000` (runbooks: `runbooks/<area>/<company>/<product>/<model>`) |
| `tags` | document-level tags only; per-topic tagging moves into sections (below) |
| `version` | the vendor doc/firmware applicability: `BIG-IP 16.1 / 17.x` |

### Section tags

Inside the body, every `##` heading is a **section**. A section carries its
*own* tags via an HTML-comment marker placed **directly under the heading**:

```markdown
## LTM — pools and monitors
<!-- section-tags: ltm, pools, monitors -->

…section content…
```

- The marker is invisible in rendered Markdown and is stripped by the parser.
- A marker that is not directly under a `##` heading is ordinary content.
- `###` and deeper headings are content, not sections.
- The app exposes sections as `entity.sections: [{ title, tags, body }]`
  (preamble before the first `##` is a section with an empty title).

### Generic category-path parsing (no hardcoded lists)

Structure comes from the slug alone. `parse_category_path` (Rust,
`commands/content.rs`; also the `content_category_path` command) assigns
roles **positionally**, by path shape — never by matching vendor/section
names, so a pack can add a new company or area with zero code changes:

```
reference/vendors/<company>/<product>/<model>[/<topic>…]
    root / category / company   / device  / model [/ topic…]
reference/<area>[/<group>[/<topic>…]]
    root / category / group   [/ topic…]
runbooks/<area>[/<topic>…]
    root / area   [/ topic…]
```

UI tree levels map onto the vendors shape as
**Category → Company → Device → Model/id**; only the Model/id (deepest)
level opens a document — upper levels are pure expand/collapse navigation.
Frontends must derive display names by prettifying slugs, never from
lookup tables.

### Migration status

- Slice 1 (dev branch): generic category-path parser + section extraction in
  the app; pilots — `vendor-f5-bigip` (`content/bigip-4000.md`) and a
  FortiGate runbook document.
- Slice 2 (dev branch): **all addons packs migrated** — 227 per-command files
  merged into 24 model documents (97 tagged sections at the time), old files
  removed after title/content verification; registry zips and `index.json`
  rebuilt at pack version 1.2.0 on the addons repo's dev branch.
- Slice 3 (dev branch): **core pack migrated** — `scripts/gen_core_pack.mjs`
  now generates three command documents (Linux / Windows / Splunk SPL,
  148 entries, cheatsheet prose sections included); the 12 handwritten
  reference files became 6 documents grouped by category; the 3 error
  documents were converted in place, keeping their stable `err/…` ids;
  core manifest bumped to 0.3.0. `scripts/verify_v2_docs.mjs` now covers
  addons + core: 38 documents, 137 section-tags markers, all ids unique.
- Still **draft**: promotion to final waits on the dev branches being pushed
  with PRs and on a full `cargo test` + visual pass on a machine with
  Tauri's system dependencies. Hardware-model splitting (e.g. ASA 5506-X
  vs 5516-X as separate documents) also remains open — it needs per-model
  source content.

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
