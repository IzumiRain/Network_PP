# Contributing to Network++

Thanks for helping build the offline IT field toolkit. There are two very
different ways to contribute:

## 1. Content — write a pack (most valuable!)

Specialized knowledge ships as **content packs**, not app code. A pack is a
zip of Markdown files with TOML frontmatter — no Rust knowledge needed.

- Read **[docs/PACK-SPEC.md](docs/PACK-SPEC.md)** — the full format spec
- Look at `packs-src/core/` for a real, working example
- Publish your own registry (GitHub repo + index.json) or open a PR adding
  your pack to the community registry

Priority areas: vendor CLI references (Cisco, MikroTik, Fortinet…),
runbooks, error references, protocol deep-dives.

Rules that keep content trustworthy:

- **One ID per fact, forever.** Categories and packs reference IDs; they
  never duplicate content.
- **Classify risk.** Every executable command carries `risk` (info/low/
  medium/high).
- **Version and source your commands.** A stale command in production hurts
  someone.

## 2. Code

Stack: **Rust + Tauri 2** backend, **vanilla TypeScript** frontend (no
framework — this is deliberate, keep it that way), SQLite/FTS5 storage.

```bash
npm install
npm run tauri dev       # desktop shell with hot reload
cargo test              # unit tests (src-tauri)
npm run typecheck       # frontend strict typecheck
cargo clippy --all-targets   # must be warning-free (CI enforces -D warnings)
```

Before opening a PR:

- tests for any new backend logic (wire formats, parsers, math get real
  fixtures)
- the Playwright UI audit mindset: render it, look at it, desktop **and**
  390 px wide — no horizontal overflow, centered things actually centered
- design follows `docs/DESIGN.md` (tokens, 8-pt grid, 8 interaction states)
- UI copy in English, short and imperative

## Reporting bugs

Open an issue with the app version (`Settings → About`), OS, and what you
expected vs. saw. For content errors, name the entity ID.

## License

MIT — by contributing you agree your work is released under the project
license.
