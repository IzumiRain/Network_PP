---
id = "doc/juniper/juniper/juniper"
kind = "vendor"
title = "Juniper Junos OS"
category = "reference/vendors/juniper"
tags = ["juniper", "junos", "cli", "config", "diagnostics", "support", "lldp", "topology", "interfaces", "bgp", "routing", "ospf", "model-doc"]
risk = "medium"
version = "Junos OS 22.x"
---
The complete Juniper Junos OS document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Config
<!-- section-tags: juniper, junos, cli, config -->


### commit (configuration mode)

Configuration-mode command that validates and activates all staged configuration changes.

#### Command

```text
commit
commit confirmed 5
commit comment "change ticket 1234"
```

#### Notes

- Nothing changes on the device until you commit; `commit check` validates without activating.
- `commit confirmed 5` auto-reverts after 5 minutes unless you commit again - the remote-safe way to test risky changes.
- `commit comment` logs the change reason into the commit history.
- `show system commit` lists past commits with time, user and comment.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### rollback (configuration mode)

Configuration-mode command that loads a previously committed configuration into the candidate.

#### Command

```text
rollback 1
commit
```

#### Notes

- `rollback 0` loads the currently committed config (discarding uncommitted edits); `rollback 1` the one before it.
- Takes effect only after a following `commit` - the device stays unchanged until then.
- `show system commit` shows which rollback number maps to which change.
- For remote work, pair with `commit confirmed` so a bad change reverts automatically.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### show configuration

Operational-mode command that prints the candidate configuration in curly-brace format.

#### Command

```text
show configuration
show configuration interfaces ge-0/0/0
show configuration | display set
```

#### Notes

- Junos applies config via a candidate model: what you see here becomes active only after `commit`.
- Narrow to a subtree by path, or use `| display set` to get copy-pasteable `set` commands.
- Diff candidate against a past commit: `show configuration | compare rollback 1`.
- Large configs output faster when scoped to one hierarchy level.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


## Diagnostics
<!-- section-tags: juniper, junos, cli, diagnostics, support, lldp, topology -->


### request support information

Collects a full technical-support dump: interfaces, protocols, chassis state and logs.

#### Command

```text
request support information
request support information | save /var/tmp/support.txt
```

#### Notes

- The output JTAC expects at the start of a case - run it early during an incident.
- Output is very long; redirect with `| save` to a file and pull it via SCP.
- Capture before restarting anything, so the broken state is preserved.
- Read-only: safe to run on a production device.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### show lldp neighbors

Lists LLDP-discovered neighbors per local interface with system name and port ID.

#### Command

```text
show lldp neighbors
show lldp neighbors interface ge-0/0/1
```

#### Notes

- Handy for building patch maps and verifying cabling between switches and routers.
- Shows chassis ID, port description and system name advertised by the peer.
- Per-interface form narrows output on large leaf/spine fabrics.
- Requires LLDP enabled (default on most modern Junos releases).

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### show version

Shows the Junos OS release, build, hostname, model and uptime.

#### Command

```text
show version
show version detail
```

#### Notes

- First check before opening a TAC case, upgrading, or matching release notes.
- `detail` also lists installed packages and jail versions.
- On mixed chassis (MX with SCB/fabric) it reports software per component.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


## Interfaces
<!-- section-tags: juniper, junos, cli, interfaces -->


### show interfaces terse

One-line status for every interface: link state, admin state and configured addresses.

#### Command

```text
show interfaces terse
show interfaces ge-0/0/0 terse
```

#### Notes

- Flags show the link (up/down) and admin state; protocol lines list IPv4/IPv6 addresses.
- Filter to one interface by name to speed up output on chassis with many ports.
- For errors, flapping and traffic counters use `show interfaces ge-0/0/0 extensive`.
- Works the same on EX, MX, SRX and QFX running Junos.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


## Routing
<!-- section-tags: juniper, junos, cli, bgp, routing, ospf -->


### show bgp summary

Summarizes BGP sessions: peer counts per AS, session states and prefix counters.

#### Command

```text
show bgp summary
show bgp summary instance <instance-name>
```

#### Notes

- `EstablSz` is the number of established sessions; peers still in `Active`/`Connect` are not forming TCP.
- Per-peer columns show received/accepted/active/advertised prefix counts - useful to spot route churn.
- Per-neighbor detail and route flapping: `show bgp neighbor <address>`.
- Sessions in `Idle` often mean no route to the peer or a config/auth mismatch.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### show ospf neighbor

Lists OSPF adjacencies with interface, area, neighbor address and adjacency state.

#### Command

```text
show ospf neighbor
show ospf neighbor extensive
```

#### Notes

- Expect `Full` on established adjacencies; anything else deserves a look.
- Stuck in `ExStart`/`Exchange` usually means an MTU mismatch; `Init` points at hello/auth or access-list issues.
- For OSPFv3 use the equivalent `show ospf3 neighbor`.
- `extensive` adds timers, DR/BDR role and retransmission counters.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation


### show route

Displays the Junos routing tables, by default the IPv4 unicast table inet.0.

#### Command

```text
show route
show route protocol ospf
show route 10.0.0.0/8
```

#### Notes

- The active route for each prefix is marked with `*`.
- Filter by protocol (ospf, bgp, static, direct), by prefix, or use `show route terse` for a compact view.
- IPv6 routes live in table inet6.0: `show route table inet6.0`.
- Routes shown as `Unusable` or hidden indicate a failed next-hop resolution.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation

