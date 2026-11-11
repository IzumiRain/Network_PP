---
id = "doc/cisco/asa/asa"
kind = "vendor"
title = "Cisco ASA"
category = "reference/vendors/cisco/asa"
tags = ["asa", "cisco", "cli", "acl", "config", "failover", "ha", "interfaces", "nat", "system", "ntp", "performance", "troubleshooting", "capture", "files", "logging", "vpn", "ipsec", "ike", "model-doc"]
risk = "medium"
version = "ASA 9.16"
---
The complete Cisco ASA document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## ACL
<!-- section-tags: asa, cisco, cli, acl, config -->


### access-list / access-group (interface ACL)

Creates an extended ACL and binds it to an interface direction with access-group.

#### Command

```cisco
access-list INSIDE_IN extended permit tcp 10.1.1.0 255.255.255.0 any eq 443
access-list INSIDE_IN extended deny ip any any log
access-group INSIDE_IN in interface inside
```

#### Notes

- The first `access-group ... in interface <name>` on a formerly unfiltered interface immediately denies everything not permitted - it is service-affecting.
- Entries cannot be reordered easily; new ACEs append at the end unless inserted by line number (`line <n>`).
- Use `no access-list INSIDE_IN line 2` to remove a single entry; `no access-list <name>` deletes the whole ACL.
- An empty or missing ACL on an interface falls back to security-level defaults.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### show access-list

Shows ACL entries with per-ACE hit counters - the ASA firewall policy in effect.

#### Command

```cisco
show access-list
show access-list INSIDE_IN
```

#### Notes

- Hit counts tell you whether a rule ever matched; a silent rule above a general permit may be dead config.
- The implicit `deny any any` at the end does not show a counter - use `packet-tracer` to prove a deny.
- Crypto ACLs for old-style L2L VPNs also appear here; their hits come through the tunnel, not the interface.
- `show run access-list` shows the configured text, while this shows counters with expanded line numbers.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


## Failover
<!-- section-tags: asa, cisco, cli, failover, ha -->


### show failover

Failover pair status: active/standby roles, monitored interfaces and state links.

#### Command

```cisco
show failover
show failover history
show monitor-interface
```

#### Notes

- First line shows this unit's role (Active/Standby Ready) and the peer's; `Other host: Primary/Secondary` follows.
- Interfaces must list as Monitored and Normal; a Waiting/Failed monitored interface drives failover decisions.
- `show failover history` lists state transitions with reasons - the fastest route to why the pair flipped.
- Failover link (stateful or not) and the LAN-based failover interface health are shown at the end of the output.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


## Interfaces
<!-- section-tags: asa, cisco, cli, interfaces, config -->


### nameif and security-level (interface config)

Names an interface and assigns its security level, which drives default trust between zones.

#### Command

```cisco
interface GigabitEthernet0/1
 nameif inside
 security-level 100
 ip address 10.1.1.1 255.255.255.0
```

#### Notes

- Defaults: inside = 100, outside = 0, others = 50; higher can reach lower unless blocked, lower cannot reach higher.
- Once an interface has an access-group ACL, the ACL (not the security level) decides traffic in that direction.
- Same-security traffic is denied between two equal-level interfaces unless `same-security-traffic permit inter-interface` is set.
- Removing `nameif` or changing the IP drops existing connections on that interface.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### show interface ip brief

One-line-per-interface summary of IP address, status and line protocol state.

#### Command

```cisco
show interface ip brief
```

#### Notes

- `up/up` means administratively up and carrier detected; `up/down` usually means wrong speed/duplex or no peer.
- Use for a quick health pass before deeper checks like `show interface GigabitEthernet0/1` (counters, errors).
- Subinterfaces appear with their VLAN tags; a missing dot1Q tag is a common cause of `up/down`.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


## NAT
<!-- section-tags: asa, cisco, cli, nat, config -->


### object network (NAT/ACL objects)

Defines reusable network objects (host, subnet, range) used by NAT rules and ACLs.

#### Command

```cisco
object network WEB_SERVER
 host 10.2.2.10
object network WEB_NET
 subnet 10.2.2.0 255.255.255.0
nat (inside,outside) static 203.0.113.10
```

#### Notes

- Objects must be created before they are referenced by `object network OBJ` NAT or `object-group`/ACL entries.
- A `nat` statement inside `object network` creates object (section 2) auto NAT.
- Deleting or editing an object referenced by NAT/ACLs changes live policy immediately - change windows apply.
- `show run object` dumps all object definitions for review.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### show nat

Displays the NAT policy table (Section 1/2/3 order) with per-rule translation and hit counters.

#### Command

```cisco
show nat
show nat detail
```

#### Notes

- Rules are evaluated top to bottom: manual NAT (section 1), object NAT (section 2), after-object NAT (section 3).
- Hit counts (translate/hit) reveal whether a rule actually matches; 0 hits on a path you expected is a clue.
- `show nat detail` appends the per-rule translation and creation flags (identity, net-to-net, unidirectional).
- First place to look when a connection is denied for no-match or unexpected translation in `packet-tracer`.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### show xlate

Shows the active NAT translation (xlate) table.

#### Command

```cisco
show xlate
show xlate detail
```

#### Notes

- Confirms a translation exists for a live flow; missing xlate often means the NAT rule never matched.
- Use `show xlate local 10.1.1.25` (or `interface inside`) to find what an internal host is translated to.
- `clear xlate` tears down existing translations and drops their flows - disruptive, never run casually.
- Stale xlates normally expire with idle timeouts; many leftover statics after config churn may need cleanup.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


## System
<!-- section-tags: asa, cisco, cli, system, ntp, performance -->


### show clock

Shows the firewall's current date, time and timezone; `detail` adds the time source.

#### Command

```cisco
show clock
show clock detail
```

#### Notes

- Wrong clocks break log correlation, certificate validity checks and VPN rekeys.
- `show clock detail` shows whether the time is authoritative or synced from NTP.
- Check `ntp server` / `show ntp status` when the clock drifts or shows an NTP fallback source.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### show cpu usage

Shows CPU utilization over 5-second, 60-second and 5-minute intervals.

#### Command

```cisco
show cpu usage
```

#### Notes

- Sustained high CPU is usually check-inspection load, logging volume or a traffic anomaly.
- Compare the 5-second vs 5-minute values: a spike pattern points to bursts, a flat 100% to overload.
- Follow up with `show processes cpu-hog` to find processes that held the CPU for long stretches.
- Multi-core platforms show a total across cores; per-core detail is available in ASDM/monitoring.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### show memory

Shows total, used and free physical memory plus allocation summary.

#### Command

```cisco
show memory
```

#### Notes

- The ASA allocator grabs most free memory by design; low free memory alone is not a fault.
- Watch the trend instead: repeated growth without recovery suggests a leak or connection-table growth.
- Use `show memory detail` and `show blocks` to inspect pools when connections are dropped for lack of memory.
- Compare before/after enabling new inspections or logging levels to gauge their cost.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### show version

Shows the ASA software image, uptime, serial number, licenses and a summary of flash contents.

#### Command

```cisco
show version
```

#### Notes

- Confirms the exact running image and platform before upgrades or a TAC case.
- Lists the chassis serial number and activation-key licenses (Security Plus, AnyConnect counts).
- Uptime and last reload reason appear here; pair with `show crashinfo` after an unexpected reboot.
- On Firepower (Secure Firewall) appliances, the FXOS version is shown alongside the ASA version.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


## Troubleshooting
<!-- section-tags: asa, cisco, cli, troubleshooting, capture, files, config, logging -->


### capture / show capture (inline packet capture)

Captures packets on a named interface with an ACL-style match filter; remove captures when done.

#### Command

```cisco
capture CAP1 interface inside match ip host 10.1.1.1 host 8.8.8.8
show capture CAP1
show capture CAP1 detail
copy /pcap capture:CAP1 tftp://10.9.9.9/cap1.pcap
no capture CAP1
```

#### Notes

- The capture definition lives in the running config (shows in `show run all capture`), so always `no capture` after the session.
- Default buffers are small; use `buffer 2000000` and `packet-length`/`circular-buffer` options for longer sessions.
- Browse `https://<asa>/admin/capture/CAP1` to download or inspect a capture without CLI access.
- Heavy captures can impact CPU; scope with `match` or an access-list instead of capturing an entire interface.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### copy (file management: TFTP/SCP/HTTP)

Copies files to/from flash - software images, ASDM images, configs and capture exports.

#### Command

```cisco
copy tftp://10.9.9.9/asa9-16-3-19.bin flash:/asa9-16-3-19.bin
copy run tftp://10.9.9.9/asa-config.cfg
boot system flash:/asa9-16-3-19.bin
```

#### Notes

- Copying a file over the current boot image is destructive to that file - always copy to a new name, `dir` to verify, then set `boot system`.
- After changing the boot variable, save with `write memory` so the change survives reload; confirm with `show version` (boot image line).
- Config backups: `copy running-config tftp://...`; restores are done by editing or `configure terminal` merge, not a blind overwrite.
- Both source and destination are URL-style; use `copy /pcap capture:CAP1 ...` for capture exports.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### dir flash: (list files)

Lists files on the internal flash (disk0:) - images, ASDM, crashinfo, backups.

#### Command

```cisco
dir flash:
dir flash:/log
show flash:
```

#### Notes

- `flash:` and `disk0:` refer to the same storage on ASA platforms; `show flash:` is an equivalent alias.
- Verify the exact image filename here before changing the boot variable or upgrading.
- Crashinfo files live under flash:/log - a growing pile of crash dumps signals repeated reloads worth investigating.
- Check free space before `copy` operations; a full flash blocks image and config uploads.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### packet-tracer (simulated path)

Simulates a packet through the ASA data path and reports the first drop phase - the key diagnostic for why traffic is blocked.

#### Command

```cisco
packet-tracer input inside tcp 10.1.1.1 12345 8.8.8.8 443
packet-tracer input inside udp 10.1.1.1 12345 8.8.8.8 53 detailed
```

#### Notes

- The output lists every phase (ACL, NAT, inspect, crypto, flow creation); the phase marked (drop) names the cause.
- Use `detailed` for per-phase explanation and the exact rule/ACE that matched.
- Nothing is really sent: it is a simulation, so it is safe in production and works even for flows that do not exist.
- The source must be reachable via the named ingress interface; only unicast protocols (tcp/udp/icmp) can be traced.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


### show logging

Shows logging configuration state, enabled destinations and the buffered log tail.

#### Command

```cisco
show logging
logging buffered informational
logging host inside 10.9.9.9
```

#### Notes

- Confirms whether logging is enabled and at what severity each destination (buffered, syslog server, ASDM) is set.
- The buffered log prints inline below the config summary - filter with `show logging | include 106023` for deny hits.
- Message 106023 (ACL deny) and 302013/302014 (flow build/teardown) are the everyday troubleshooting messages.
- Use `logging list <name> <level|class>` to define what a syslog destination receives; avoid debug levels in production.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com


## VPN
<!-- section-tags: asa, cisco, cli, vpn, ipsec, ike -->


### show crypto ipsec sa

IPsec SA details: phase-2 proposals, encaps/decaps counters and interesting-traffic ACL.

#### Command

```cisco
show crypto ipsec sa
show crypto ipsec sa peer 203.0.113.5
show crypto ipsec sa detail
```

#### Notes

- Rising encap AND decap counters prove live two-way traffic; encap without decap usually means a return-path/ACL/NAT issue.
- `pkts encaps`/`pkts decaps` near zero with an active SA means no interesting traffic is matching.
- Rekey attempts and errors (e.g. `sa created`, `route-metric`) show in the peer block.
- Old crypto-map L2L designs gate phase 2 on a crypto ACL; mismatched networks on either side show as no-SA.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### show crypto isakmp sa

Shows IKE (phase 1) security-association state with peers and negotiation status.

#### Command

```cisco
show crypto isakmp sa
show crypto isakmp sa detail
```

#### Notes

- `MM_ACTIVE` means phase 1 is up; states stuck in MM_NO_STATE / MM_KEY_EXCH usually mean ISAKMP policy or UDP 500/4500 mismatch.
- `MM_WAIT_MSG_6`-style hangs often point to authentication (PSK/cert) or proposal failures.
- IKEv2 sessions appear in the same output; `detail` adds local/remote identities and durations.
- Check `show run crypto isakmp` (or `ikev2`) for the policy and `debug crypto ikev1/ikev2 <peer>` when SAs never form.

Source: Cisco ASA Series Command Reference — https://www.cisco.com


### show vpn-sessiondb

VPN session database: summary, per-user (AnyConnect) and site-to-site tunnel details.

#### Command

```cisco
show vpn-sessiondb summary
show vpn-sessiondb anyconnect
show vpn-sessiondb l2l
show vpn-sessiondb anyconnect filter name jdoe
```

#### Notes

- `summary` is the first stop for counts by type and license/limit usage.
- `l2l` entries show the peer IP, tunnel group and assigned networks per site-to-site tunnel.
- `filter name <user>` and `sort by` narrow long user lists during a support call.
- Session limits hitting capacity show here as denied session attempts before users report failures.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com

