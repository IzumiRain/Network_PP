---
id = "doc/check-point/check-point/check-point"
kind = "vendor"
title = "Check Point Gaia"
category = "reference/vendors/check-point"
tags = ["checkpoint", "cli", "diagnostics", "services", "gaia", "performance", "kernel", "capture", "troubleshooting", "clish", "hardware", "interfaces", "model-doc"]
risk = "high"
version = "Gaia R81.x"
---
The complete Check Point Gaia document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Check Point
<!-- section-tags: checkpoint, cli, diagnostics, services, gaia, performance, kernel, capture, troubleshooting, clish, hardware, interfaces -->


### cpstat (status probes)

Compact one-shot status probes for OS health and gateway components.

#### Command

```text
cpstat os -f all
cpstat fw
```

#### Notes

- `cpstat os -f all` reports OS-level health: CPU, memory, disks and hardware sensors.
- `cpstat fw` reports security gateway status in a compact form.
- Output is short and scriptable - handy for cron-based health checks.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### cpstop / cpstart

Stops or starts all Check Point security product daemons on the machine.

#### Command

```text
cpstop
cpstart
```

#### Notes

- On a standalone gateway without HA, cpstop halts traffic handling immediately.
- In a cluster, stopping one member triggers failover - still announce the maintenance.
- Bring everything back with cpstart and verify with cpview or `cpstat os -f all`.
- Run from Expert mode; never leave a gateway stopped after a troubleshooting session.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### cpview (performance view)

Launches the Check Point performance view TUI: CPU, memory, connections, throughput, VPN.

#### Command

```text
cpview
```

#### Notes

- Best first tool for "gateway is slow" triage on Gaia.
- Navigate the metric tree with the on-screen menu; exit from the File menu.
- Watch FW connections vs. capacity and CPU per core under load.
- Works locally via SSH and is read-only.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### fw ctl pstat

Shows kernel inspection statistics: free kernel memory, connections, NAT and VPN.

#### Command

```text
fw ctl pstat
```

#### Notes

- Run from Expert mode (enter `expert` from clish first).
- Watch `Free` kernel memory and the connections hash usage under load.
- A nearly full kernel memory or hash table shows up here before users report drops.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### fw monitor (packet capture)

Captures packets as they pass the firewall inspection points.

#### Command

```text
fw monitor -e 'accept src=10.0.0.5 and dst=8.8.8.8;'
```

#### Notes

- Runs from Expert mode; shows the same packet at each inspection point (i/I/o/O).
- Ctrl+C stops the capture and prints the per-point summary.
- Add `-o <file.cap>` to write a pcap for Wireshark instead of the console.
- Keep the accept filter tight - a wide filter floods the console and skews results.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### show asset all (Gaia clish)

Shows hardware inventory: platform, serial numbers, memory and disks.

#### Command

```text
show asset all
```

#### Notes

- Needed for RMA requests and license/host registration.
- Confirms appliance model and specs before sizing or upgrade work.
- `show asset` without `all` prints the top-level summary.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### show interface (Gaia clish)

Lists interfaces, or shows detail for one interface.

#### Command

```text
show interface
show interface eth1
```

#### Notes

- The list form shows every interface with its IPv4/IPv6 and on/off state.
- The name form shows address, link state, speed/duplex and MTU.
- Configuration happens in the same clish via `set interface ...` followed by `save config`.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com


### show version (Gaia clish)

Shows the Gaia OS version, build number and host name.

#### Command

```text
show version
```

#### Notes

- Verify the same build on every cluster member before and after maintenance.
- Match the version against Check Point's Jumbo Hotfix Accumulator (JHF) release notes.
- GUI equivalent: Gaia Portal home page overview.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com

