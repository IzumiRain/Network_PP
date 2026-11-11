---
id = "doc/runbooks/os-linux"
kind = "runbook"
title = "Runbooks: Linux"
category = "runbooks/os-linux"
tags = ["linux", "storage", "troubleshooting", "model-doc"]
risk = "info"
version = "core-0.3.0"
---
Every os-linux runbook in one document (v2 document model — see
docs/PACK-SPEC.md). Each runbook is a section carrying its own tags.


## Disk full on a Linux server
<!-- section-tags: linux, storage, troubleshooting -->

#### Prerequisites

- Root or sudo
- Know what runs on the host (databases delete themselves carefully)

#### Steps

- [ ] 1. Confirm and locate: `df -h` — which filesystem? If `df` shows free space but writes still fail, run `df -i` (inodes exhausted — usually millions of tiny files)
- [ ] 2. Find the big consumers: `du -xh --max-depth=2 / 2>/dev/null | sort -h | tail -20`
- [ ] 3. Classic suspects: `/var/log` (journald, old logs), `/var/lib/docker` (dangling images/volumes), package caches, core dumps, `/tmp`
- [ ] 4. Safe wins first: `journalctl --vacuum-size=200M`, old `*.log*.gz`, `docker system prune` (read what it deletes first), package cache clean (`apt clean` / `dnf clean all`)
- [ ] 5. Deleted-but-open files: `lsof +L1` — a service holds a huge deleted log; restart that service to release the space
- [ ] 6. Check for runaways: a single runaway app log — fix the rotation (`logrotate` config), not just the symptom
- [ ] 7. Only then consider growth: extend LVM (`lvextend -r -L +5G /dev/vg/lv`) or add a disk — confirm the volume actually has space on the VG first (`vgs`)

#### Validation

- [ ] `df -h` shows healthy headroom (target: < 80% used)
- [ ] The application writes again
- [ ] Log rotation in place for whatever you truncated

#### Rollback

- Everything deleted in steps 4–6 is disposable by design; if you trimmed a
  log the app still writes, nothing is lost. Do NOT delete files under
  `/var/lib/{docker,mysql,postgresql,...}` by hand to free space — use the
  tool's own maintenance commands or backups.

#### Risk notes

- `rm -rf` paths: paste, re-read, then run. No globs on the root level.
- On databases: never free space by deleting live data files — corrupts the
  cluster; use engine commands (`VACUUM`, binlog expiry).
