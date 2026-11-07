# Security Policy

## Supported versions

| Version | Supported |
|---|---|
| 1.2.x | ✅ |
| < 1.2 | ❌ (upgrade) |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Use GitHub's private vulnerability reporting:
**Security → Report a vulnerability** on this repository, or contact the
maintainer directly if you prefer another channel.

Include what you can of:

- affected version (`Settings → About`) and platform
- the trust boundary crossed (e.g. webview → backend command, pack
  install path, network response handling)
- minimal reproduction steps or a proof of concept
- your assessment of impact

You will get an acknowledgement within **7 days**. Fixes ship as patch
releases; you will be credited in the release notes if you wish.

## Scope notes

Network++ is a local desktop/mobile application. Areas of interest include:

- **Tauri IPC surface** — every command in `src-tauri/src/lib.rs` and
  submodules; the webview holds `invoke`, so any stored XSS or
  command-argument injection is a full backend-access bug
- **Content pack pipeline** — `packs.rs` (zip extraction, checksums,
  path handling), `content.rs` (frontmatter parsing); packs are
  data-only by design, so anything that lets a pack execute code is
  critical
- **Network response handling** — DNS/HTTP/PCAP parsers, banner and
  header rendering
- **Update flow** — `updatecmds.rs`; the installer executor allowlists
  official release URLs by design

Out of scope: the scanning features being used against systems without
authorization (that is a license violation, not a vulnerability — see
the legal-use section in [LICENSE](LICENSE)), and vulnerabilities in
dependencies without a demonstrated path into this application.

## Hardening history

Security-relevant fixes are tagged in release notes
([docs/releases](docs/releases)): e.g. 1.0.1 (XSS escaping across all
views, ping argument sanitization), 1.2.0 (update-flow URL allowlist,
verbatim license text), 1.2.1 (update-download authentication).
