<div align="center">

# Network++

**A lightweight network engineering toolkit — calculators, packet visualizer, cheat sheets and scanners.**

Built with Rust + Tauri 2. Desktop today, Android next.

</div>

## Why

Most networking tools are either heavy GUI suites or scattered CLI one-liners.
Network++ puts the daily-use parts — subnet math, port lookup, packet anatomy,
command cheat sheets and quick scans — into one small, offline, native app that
runs on weak hardware.

## What ships in 1.0.0


**New IA** — Home / Reference / Tools / Diagnostics / Runbooks / Settings, with global search (Ctrl+K),
favorites, pin-to-home, history and a local SQLite (FTS5) content index.

| Module | What it does |
|---|---|
| **Reference** | Content-pack driven knowledge base: commands (Linux/Windows/SPL), networking fundamentals, HTTP/network/TLS errors, standards — searchable, versioned, risk-classified |
| **Runbooks** | Operational checklists with steps, decisions and validation (no-connectivity, DNS failures, disk-full) |
| **Content packs** | Install/remove specialized content: registry download (SHA-256 verified, jsDelivr-friendly) or offline file import; manual SOCKS5/HTTP proxy support |
| **CIDR & IP range** | Network/broadcast/host range, wildcard mask, class & type, subnet splitting, range→CIDR, membership check — live as you type |
| **Ports** | TCP/UDP lookup for 100+ assignments, service search, range classification |
| **Packet visualizer** | Build IPv4/TCP packets field-by-field with live byte output, or paste a hex dump and read every layer (Ethernet / IPv4 / IPv6 / TCP / UDP / ICMP / HTTP) with real checksums |
| **Target scan** | nmap-style TCP connect scan with concurrency, banner grabbing and service names |
| **Local devices** | ARP table reader with vendor identification, TCP host discovery, system ping sweep |
| **Connections** | Live TCP/UDP socket table with state filters and auto-refresh |

**Diagnostics suite:** a pure-Rust DNS client (9 record types, resolver
comparison, RCODE reporting), HTTP probes with full header tables, system
diagnostics (disks, processes, host info) and **diagnostic sessions** —
group tests per problem, record results with timestamps, export a clean
Markdown report.

**PCAP reading:** classic pcap files (both byte orders, µs/ns) parsed in
Rust with per-packet L2/L3/L4 summaries — fully local.

**Interactive runbooks:** checkbox progress persists per runbook in SQLite.

**Tools expansion:** IPv6 calculator (RFC 5952, EUI-64), transfer time/MTU/MSS
and packet-overhead calculators, number/IP/MAC/Base64/URL/hex/timestamp
converters, UUID/MAC/IP/password/hash generators, and text tools (JSON
formatter, regex tester, line diff, case & whitespace).

## Status & roadmap

- [x] 0.1 — network toolkit (calculators, packet lab, scanning, cheatsheets)
- [x] 0.2 — product foundation: new IA, content engine (9 templates), SQLite+FTS5 search, favorites/pins/history, content-pack system with registry + offline import, core pack
- [x] 0.3 — tool expansion: IPv6, bandwidth/MTU, converters, generators, formatters
- [x] 0.4 — diagnostics suite: DNS client, HTTP tests, system info, diagnostic sessions
- [x] 0.5 — PCAP reading + interactive runbooks (persisted progress)
- [x] 1.0.0 — first stable release: polish, packaging, docs, release CI
- [ ] 1.x — Android build (SDK/NDK), then raw capture on desktops

## Development

Prerequisites: **Rust** (stable, MSVC on Windows), **Node.js 20+**, and on
Windows the WebView2 runtime (preinstalled on Win10/11).

```bash
npm install
npm run tauri dev      # desktop app with hot reload
npm run tauri build    # release build + installer
cargo test             # Rust unit tests (in src-tauri/)
npm run typecheck      # frontend type check
```

## Architecture

```
src/            frontend — vanilla TypeScript, no framework, ~54 KB JS
src-tauri/      backend — Rust, std-first, no heavy network deps
docs/DESIGN.md  design system contract (tokens, components, states)
```

The Rust backend implements CIDR math, packet parse/build with checksums,
scanning and connection listing from first principles — no nmap/wireshark
binaries required. Scanning is capped (65,536 hosts, 2,048 ports per scan) and
runs from a bounded thread pool.

## Legal note

Network++ is a toolkit. Scanning hosts you do not own or lack permission for is
illegal in most jurisdictions. Use it on your own networks and lab equipment.

## License

MIT — see [LICENSE](LICENSE).
