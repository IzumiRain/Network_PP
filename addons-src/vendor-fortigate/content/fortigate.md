---
id = "doc/fortinet/fortigate/fortigate"
kind = "vendor"
title = "Fortinet FortiGate"
category = "reference/vendors/fortinet/fortigate"
tags = ["fortigate", "fortios", "ipsec", "ike", "logging", "cli", "ha", "interfaces", "policies", "sessions", "firmware", "performance", "dns", "backup", "updates", "system", "dhcp", "debug", "model-doc"]
risk = "medium"
version = "FortiOS 7.4"
---
The complete Fortinet FortiGate document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Errors
<!-- section-tags: fortigate, fortios, ipsec, ike, logging -->


### IPsec phase 2 fails with no proposal chosen

The tunnel completes IKE phase 1 but every phase 2 negotiation fails and the event log reports no proposal chosen (IKEv2 NO_PROPOSAL_CHOSEN).

#### Common causes

- Phase 2 proposal mismatch: encryption, hash or DH-group (PFS) values differ between the two peers.
- One side has PFS enabled and the other disabled.
- Phase 2 traffic selectors do not cover the actual source/destination subnets being negotiated.

#### Diagnose

```cisco
diagnose vpn ike log-filter dst-addr4 203.0.113.2
diagnose debug application ike -1
diagnose debug enable
```

#### Notes

- The IKE debug output shows both proposals side by side; align them (and the selectors) on both peers.
- Remember to run diagnose debug disable afterwards.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### Log disk full on FortiGate

The FortiGate reports a disk full condition: local logging stops or starts overwriting, and the Log and Report page shows warnings.

#### Common causes

- Too many log categories enabled at information or debug severity for a small flash/SSD.
- Long local retention with no log offloading, so old logs are never purged.
- A single noisy category (for example full traffic logging) filling the disk fast.

#### Fix

- Reduce the enabled log categories and raise the severity threshold of noisy sources.
- Offload logs to FortiAnalyzer/SIEM instead of keeping everything on the firewall:

```cisco
config log fortianalyzer setting
    set status enable
end
```

- Delete old logs from the Log Settings page in the GUI once retention needs are met elsewhere.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


## HA
<!-- section-tags: fortigate, fortios, cli, ha -->


### get system ha status

Shows the HA cluster state: mode, group id, primary/secondary roles, priorities, serials and sync status per member.

#### Command

```cisco
get system ha status
```

#### Notes

- Always confirm which member is primary here before any maintenance or reboot in an HA pair.
- The member list shows priority, uptime and serial number; mismatches hint at split-brain or stale units.
- Pair with show system ha to see the full HA configuration (mode, override, heartbeat interfaces).

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


## Interfaces
<!-- section-tags: fortigate, fortios, cli, interfaces -->


### diagnose hardware deviceinfo nic

Shows driver-level NIC details: link speed and duplex, media, MAC and packet/error counters for the named interface.

#### Command

```cisco
diagnose hardware deviceinfo nic port1
diagnose hardware deviceinfo nic
```

#### Notes

- Run with an interface name for one NIC; without a name it lists every NIC on the box.
- Link down here while show system interface still shows the IP configured points to cabling or switch port problems.
- Rising error/drop counters usually mean duplex mismatch, a bad cable, or an overloaded interface.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### show system interface

Prints the layer-3 configuration of every interface: mode and address, speed/duplex, role, MTU and management access.

#### Command

```cisco
show system interface
show system interface port1
```

#### Notes

- Check mode (static vs dhcp), the IP/mask and secondary IPs first when an interface looks wrong.
- allowaccess lists which management services (ping https ssh snmp) are open on the interface.
- Pair with diagnose hardware deviceinfo nic for the physical link state underneath.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


## Policies
<!-- section-tags: fortigate, fortios, cli, policies -->


### show firewall policy

Dumps the firewall policy table in evaluation order, including source/destination interfaces, addresses, services, NAT and profiles.

#### Command

```cisco
show firewall policy
show firewall policy 12
```

#### Notes

- Policies are evaluated top-down and the first match wins, so output order is the processing order.
- With a policy id the command prints a single rule — use the id seen in debug flow output.
- Verify srcintf/dstintf pairs: a rule never matches if traffic enters/leaves other interfaces.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


## Sessions
<!-- section-tags: fortigate, fortios, cli, sessions -->


### diagnose sys session filter

Sets the match criteria used by diagnose sys session list (and session clear); multiple filters combine with AND.

#### Command

```cisco
diagnose sys session filter dst 10.20.0.15
diagnose sys session filter dport 443
diagnose sys session filter proto 6
diagnose sys session filter clear
```

#### Notes

- Filters apply to the CLI session only and persist until cleared — clear them before each new investigation.
- Common keys: dst/src, dport/sport, proto, policy; an empty filter set matches every session.
- Changing the filter does not touch firewall configuration, only what the CLI displays.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### diagnose sys session list

Dumps the live session table: endpoints, protocol and state, NAT translation, expiry and the matching policy id.

#### Command

```cisco
diagnose sys session list
```

#### Notes

- Always narrow with diagnose sys session filter first — the full table is huge on a busy firewall.
- The policy id field tells you which firewall rule the flow actually hit.
- NAT fields show the translated address/port, useful to verify SNAT/DNAT behavior.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


## System
<!-- section-tags: fortigate, fortios, cli, firmware, performance, dns, backup, updates, system, dhcp -->


### diagnose sys flash list

Lists the flash partitions and installed firmware images with their boot partition assignment.

#### Command

```cisco
diagnose sys flash list
```

#### Notes

- Shows partition names, sizes, and which image is marked as the current boot image.
- Use it after an upgrade to confirm which firmware actually booted, and before a downgrade to check free space.
- Read-only; complements the Version line from get system status.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### diagnose sys top

Live CPU and memory view per process, refreshed continuously until you stop it.

#### Command

```cisco
diagnose sys top 5 10
```

#### Notes

- Arguments are refresh interval and refresh count: this samples every 5 seconds for 10 rounds.
- The header shows load average and the CPU split (us/sy/wa/hi) plus memory usage.
- Look for a daemon pinned at ~100% CPU (for example ipsupd or scanupd) when the unit feels slow.
- Stop output with Ctrl+C; run it over SSH, not the console, on busy units.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### diagnose test application dnsserver

Runs a live query test against the configured DNS servers and reports the response time per server.

#### Command

```cisco
diagnose test application dnsserver 3
```

#### Notes

- The option value selects the test action; 3 queries each configured DNS server and prints its RTT.
- Use it when DNS-dependent features (web filtering, FortiGuard lookups) are slow or timing out.
- Requires DNS servers set under config system dns; a missing RTT means that server is unreachable.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### execute backup config

Pushes a copy of the running configuration to an external server (TFTP shown; FTP variant is available).

#### Command

```cisco
execute backup config tftp fgt-backup.conf 10.0.0.50
```

#### Notes

- The first argument is the file name on the TFTP server, the second the server IP.
- Take an off-box backup before every upgrade or risky change; it is the fastest rollback artifact.
- The file contains the full config including encrypted secrets, so store it on a trusted server.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### execute update-now

Triggers an immediate FortiGuard update pull (AV, IPS, web filter databases) instead of waiting for the scheduled update.

#### Command

```cisco
execute update-now
```

#### Notes

- Downloads and installs new signature versions, so it changes protection DB state on the unit.
- Causes brief disk and CPU activity; avoid running it during traffic peaks on small models.
- Verify the result afterwards in the FortiGuard dashboard or the event log.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### get system status

Shows the appliance summary: firmware version and build, serial number, BIOS, uptime, CPU and memory.

#### Command

```cisco
get system status
```

#### Notes

- First command to run on any FortiGate session; confirms the firmware build before you apply changes.
- Serial number and uptime are required for most FortiCare/TAC cases.
- Compare the Version line with the images listed by diagnose sys flash list after an upgrade.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### show system dhcp server

Lists the DHCP server configuration per interface: ranges, lease time, gateway and DNS options.

#### Command

```cisco
show system dhcp server
```

#### Notes

- Shows the start/end IP ranges, netmask, default-gateway and dns-server handed out on each interface.
- Verify range boundaries and reserved (MAC-mapped) IPs when clients receive wrong or overlapping addresses.
- The CLI shows configuration only; active lease state is inspected in the GUI or via the dhcpd daemon.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### show system global

Dumps the running global system configuration: hostname, admin ports/timeouts, DNS, NTP and aliases.

#### Command

```cisco
show system global
```

#### Notes

- Check admintimeout and the admin-sport (HTTPS port) here before scripted sessions or GUI access fail mysteriously.
- Shows dns, ntp and central-management settings that affect lookups and updates fleet-wide.
- Read-only view of the config node; changes are made under config system global ... end.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


## Troubleshooting
<!-- section-tags: fortigate, fortios, cli, debug, logging -->


### diagnose debug flow

Prints live packet decisions through the FortiGate: route lookup, policy search, match, NAT and profile handoff.

#### Command

```cisco
diagnose debug reset
diagnose debug flow filter addr 10.0.0.5
diagnose debug flow trace start 10
diagnose debug enable
diagnose debug disable
```

#### Notes

- filter addr restricts output to one host; filter port and filter proto are also available, trace start 10 traces 10 packets.
- The decisive lines are the policy search result (matched policy id) and any deny or no-route verdicts.
- Always run diagnose debug disable (and reset) when finished — debug output costs CPU on busy units.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com


### execute log display

Prints locally stored logs matching the current execute log filter criteria directly to the CLI.

#### Command

```cisco
execute log display
```

#### Notes

- Quick triage path when the GUI is unreachable or you already have an SSH session open.
- Large result sets paginate slowly; narrow the filter (category, free-text, time range) first.
- Requires disk logging to be enabled; for full retention and reporting use FortiAnalyzer.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### execute log filter

Sets the criteria (log category, device, date range, free-text) that execute log display will match.

#### Command

```cisco
execute log filter category 1
execute log filter free-text 10.0.0.5
```

#### Notes

- Category 1 is traffic and category 2 is event; see the log category reference for the full numbering.
- The filter persists for later display runs, so reset or re-set it between searches to avoid stale criteria.
- Changing the filter only affects CLI log retrieval, not what the device actually logs.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


## VPN
<!-- section-tags: fortigate, fortios, cli, ipsec -->


### diagnose vpn tunnel list

Lists every IPsec tunnel with phase1/phase2 SA details, lifetimes, and rx/tx byte counters.

#### Command

```cisco
diagnose vpn tunnel list
```

#### Notes

- Byte counters growing on both sides means traffic is actually flowing through the tunnel.
- Counters stuck while the tunnel reports up usually indicate a phase2 selector or routing problem.
- Pair with diagnose vpn ike gateway list for the phase1 peer negotiation state.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com


### get vpn ipsec tunnel summary

Prints a compact one-line status per IPsec tunnel: tunnel name, state (up/down) and active SAs.

#### Command

```cisco
get vpn ipsec tunnel summary
```

#### Notes

- Fastest health overview for scripts and shift-handover notes, versus the verbose diagnose vpn tunnel list.
- A tunnel can show up while no traffic passes; confirm with counters before declaring it healthy.
- Combine with get system ha status on HA clusters to see which member owns the tunnel.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com

