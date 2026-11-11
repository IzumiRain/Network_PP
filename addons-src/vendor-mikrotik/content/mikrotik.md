---
id = "doc/mikrotik/mikrotik/mikrotik"
kind = "vendor"
title = "MikroTik RouterOS"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "backup", "config", "firewall", "interfaces", "ip", "hotspot", "captive-portal", "routing", "system", "diagnostics", "ping", "traceroute", "model-doc"]
risk = "medium"
version = "RouterOS 7.x"
---
The complete MikroTik RouterOS document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Config
<!-- section-tags: mikrotik, routeros, cli, backup, config -->


### /system backup save

Creates a binary .backup file of the full router configuration on flash storage.

#### Command

```text
/system backup save name=daily-backup
/system backup save name=daily-backup password=s3cret
```

#### Notes

- Binary backups restore the exact configuration; best for identical hardware and RouterOS version.
- With `password=` the backup is encrypted; without it secrets are stored in the clear.
- Restore with `/system backup load name=daily-backup`.
- For portable text config use `/export` instead.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


### /export (configuration export)

Prints the whole RouterOS configuration as importable .rsc commands.

#### Command

```text
/export
/export file=config-export
/export hide-sensitive
```

#### Notes

- In RouterOS 7 the plain `export` command works from the top-level menu and inside any submenu.
- With `file=` the output goes to a .rsc file on flash; retrieve it via the Files menu, SFTP or FTP.
- `hide-sensitive` omits passwords and keys - use it when sharing configs for support.
- Apply on another unit with `/import file=config-export.rsc`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


## Firewall
<!-- section-tags: mikrotik, routeros, cli, firewall -->


### /ip firewall filter add

Adds a firewall filter rule; rules take effect immediately in top-down order.

#### Command

```text
/ip firewall filter add chain=input protocol=tcp dst-port=22 src-address=10.0.0.0/24 action=accept
/ip firewall filter disable 3
```

#### Notes

- The example allows SSH from one subnet only; adapt chain, protocol and ports to your policy.
- A wrong early drop rule can cut your own management session - keep console access available.
- Test with `disable`/`enable <rule-number>` before committing to a change.
- Keep established/related accept rules ordered before catch-all drops.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


### /ip firewall filter print

Lists packet-filter rules per chain (input, forward, output) with hit counters.

#### Command

```text
/ip firewall filter print
/ip firewall filter print stats
/ip firewall filter print detail
```

#### Notes

- `print stats` adds per-rule packet/byte counters - the first stop when traffic is unexpectedly blocked.
- `X` marks a disabled rule; `D` marks dynamic rules created by RouterOS services.
- Rules are matched top-down and the first match wins.
- `print detail` shows the full rule including comments and time-based options.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


## Interface
<!-- section-tags: mikrotik, routeros, cli, interfaces -->


### /interface print

Lists all interfaces with type, link state and traffic flags.

#### Command

```text
/interface print
/interface print stats
```

#### Notes

- `R` marks a running interface (link up); no `R` means no cable/link or disabled port.
- `print stats` shows rx/tx byte and packet counters per interface.
- Ethernet-specific settings (speed, duplex, PoE) live under `/interface ethernet`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


## IP
<!-- section-tags: mikrotik, routeros, cli, ip, interfaces, hotspot, captive-portal, routing -->


### /ip address print

Shows every IP address configured on the router with its interface and status.

#### Command

```text
/ip address print
/ip address print detail
```

#### Notes

- `A` flag marks the active address; `D` marks dynamic ones (DHCP client, PPP).
- Shows address, network, interface and interface link state in one view.
- Addresses are managed in the same submenu - expect a brief outage when changing the address you are connected through.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


### /ip hotspot print (servers and users)

Shows HotSpot server interfaces, user accounts and currently active sessions.

#### Command

```text
/ip hotspot print
/ip hotspot user print
/ip hotspot active print
```

#### Notes

- `ip hotspot` lists HotSpot servers bound to interfaces; `ip hotspot user` lists accounts and quotas.
- `ip hotspot active` shows users that are currently authenticated, with IPs and uptime.
- Login-page and RADIUS settings live in `ip hotspot profile` and `ip hotspot user profile`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


### /ip route print

Displays the routing table with active and dynamic route flags.

#### Command

```text
/ip route print
/ip route print where dst-address=10.0.0.0/8
```

#### Notes

- `A` marks routes actually in use; `D` marks dynamic entries (connected, DHCP, OSPF/BGP learned).
- `0.0.0.0/0` entries are default routes - verify the expected gateway is active.
- `print where` filters (e.g. by `gateway` or `routing-table`); `print detail` shows all attributes.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


## System
<!-- section-tags: mikrotik, routeros, cli, system -->


### /system resource print

Shows router health and identity: uptime, CPU load, memory, version and board.

#### Command

```text
/system resource print
/system resource print once
```

#### Notes

- First check when a router feels slow: cpu-load, free-memory and cpu-count.
- The version line tells you which RouterOS release notes and upgrades apply.
- `print once` prints a single snapshot instead of a refreshing list.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


## Tools
<!-- section-tags: mikrotik, routeros, cli, diagnostics, ping, traceroute -->


### /ping

Sends ICMP echo requests from the router; ping is a top-level command.

#### Command

```text
/ping 8.8.8.8
/ping 8.8.8.8 count=10 size=1400
```

#### Notes

- Runs until stopped with Ctrl+C unless you pass `count=`.
- Use `size=` near path-MTU limits to hunt fragmentation problems.
- `src-address=` tests reachability from a specific local IP (useful with multiple WAN links).
- Note that traceroute lives under `/tool traceroute`, not `/tool ping`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com


### /tool traceroute

Traces the hop-by-hop path from the router to a destination.

#### Command

```text
/tool traceroute 8.8.8.8
```

#### Notes

- Shows each hop with per-probe latency; timeouts in the middle are often just rate-limited routers.
- `protocol=icmp` helps when the default UDP probes are filtered along the path.
- Run it twice and compare: intermittent paths change between runs.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com

