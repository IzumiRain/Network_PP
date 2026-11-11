---
id = "doc/pfsense-opnsense/pfsense/pfsense"
kind = "vendor"
title = "pfSense"
category = "reference/vendors/pfsense-opnsense/pfsense"
tags = ["pfsense", "opnsense", "cli", "backup", "config", "firewall", "pf", "freebsd", "nat", "logs", "packages", "model-doc"]
risk = "high"
version = "pfSense CE 2.7 / pfSense CE 2.7 / OPNsense 24.x"
---
The complete pfSense document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Config
<!-- section-tags: pfsense, opnsense, cli, backup, config -->


### back up /conf/config.xml

Copies the complete firewall configuration file to a local backup path.

#### Command

```bash
cp /conf/config.xml /root/config-backup-$(date +%F).xml
```

#### Notes

- config.xml holds the full configuration including secrets - store copies securely.
- Get it off-box with `scp admin@<firewall>:/conf/config.xml .`.
- Restore by uploading via the GUI (Diagnostics > Backup & Restore) or replacing the file and rebooting.
- Both pfSense and OPNsense keep the active config at /conf/config.xml.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### viconfig (edit config.xml)

pfSense shell wrapper that opens /conf/config.xml directly in vi.

#### Command

```bash
viconfig
```

#### Notes

- Edits raw XML with no validation - a malformed file can keep the firewall from booting cleanly.
- Changes do not reach the running filter until the config is saved/applied via the GUI or a reboot.
- Make a copy first: `cp /conf/config.xml /root/config-before-edit.xml`.
- Prefer the WebGUI or API unless following vendor or TAC instructions step by step.

Source: Netgate pfSense Documentation — https://docs.netgate.com


## Firewall
<!-- section-tags: pfsense, cli, firewall, opnsense, pf, freebsd, nat -->


### easyrule (quick block/pass from CLI)

Adds a real block or pass rule to the running configuration from the shell.

#### Command

```bash
easyrule block wan 203.0.113.50
easyrule pass lan 10.0.1.50 8.8.8.8 443 tcp
```

#### Notes

- `block` takes interface plus source; `pass` optionally adds destination, port and protocol.
- Rules created here are persistent and visible in the GUI under Firewall > Rules.
- Verify the result with `pfctl -sr` or in the WebGUI.
- Still follow the normal change process - this writes to the firewall config.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### pfctl -d / pfctl -e (disable and enable pf)

Disables or re-enables the packet filter entirely.

#### Command

```bash
pfctl -d
pfctl -e
```

#### Notes

- `pfctl -d` turns off all firewalling and NAT until pf is enabled again.
- Use only for short, controlled diagnostics on networks you fully trust.
- A GUI filter reload re-enables pf automatically, which can end your test early.
- Re-enable with `pfctl -e` and verify with `pfctl -si` afterwards.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### pfctl -sa (show everything)

Prints all pf information: rules, NAT, states, tables and statistics.

#### Command

```bash
pfctl -sa
```

#### Notes

- Equivalent of running every `-s` view at once; output is long, so pipe through grep.
- Useful as a single snapshot to attach to a ticket before and after a change.
- The states and tables sections are the ones that grow with traffic.
- Read-only: safe on production.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### pfctl -si (show info and stats)

Shows pf counters: state table usage, searches, matches, blocks and passes.

#### Command

```bash
pfctl -si
```

#### Notes

- Watch `current entries` against the state-table limit when connections fail.
- Rising block counters confirm that pf is actively dropping something.
- Good for a quick health check without dumping the whole ruleset.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### pfctl -sn (show NAT rules)

Shows the loaded NAT rules: outbound, port forwards and 1:1 mappings.

#### Command

```bash
pfctl -sn
```

#### Notes

- Shows NAT as pf sees it after translation of the WebGUI config (binat/rdr/nat rules).
- Port forwards appear as `rdr` rules on the interface they are bound to.
- When a port forward does not work, verify it exists here before touching the GUI.

Source: Netgate pfSense Documentation — https://docs.netgate.com


### pfctl -sr (show rules)

Shows the packet-filter ruleset exactly as pf has it loaded.

#### Command

```bash
pfctl -sr
```

#### Notes

- Shows what pf actually enforces, including WebGUI-generated rules wrapped in anchors.
- Compare with `pfctl -sn` for NAT rules and `pfctl -ss` for active states.
- Rules added by the GUI often appear as anchor sub-rulesets rather than flat lines.
- Read-only: safe on production.

Source: Netgate pfSense Documentation — https://docs.netgate.com


## Logs
<!-- section-tags: pfsense, opnsense, cli, logs -->


### read the firewall (filter) log

Reads the firewall packet log from the shell; paths differ by product and version.

#### Command

```bash
# pfSense CE 2.4 and earlier
clog /var/log/filter.log
# pfSense CE 2.5 and later
clog /var/log/filter/filter.log
# OPNsense
tail -f /var/log/firewall/latest.log
```

#### Notes

- pfSense logs are circular ring buffers; since 2.5 the live file is /var/log/filter/filter.log (rotated .gz copies alongside).
- OPNsense keeps the firewall log in /var/log/firewall/ as plain text, so tail/grep work directly.
- Log lines show time, interface, action (block/pass), protocol and src -> dst.

Source: Netgate pfSense Documentation — https://docs.netgate.com


## Packages
<!-- section-tags: pfsense, opnsense, freebsd, cli, packages -->


### pkg / pkg-static (install packages)

Installs FreeBSD packages via pkg; both distributions ship the vendor repo preconfigured.

#### Command

```bash
pkg install <package-name>
pkg info <package-name>
```

#### Notes

- pfSense scripts typically call the same binary as `pkg-static`; `pkg` works in the shell.
- Prefer the WebGUI package manager (System > Package Manager / Firmware > Plugins) for firewall packages.
- Manually installed packages may be removed on upgrade and are not covered by vendor support.
- `pkg info` lists what is installed before you change anything.

Source: Netgate pfSense Documentation — https://docs.netgate.com

