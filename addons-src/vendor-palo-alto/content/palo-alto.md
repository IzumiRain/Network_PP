---
id = "doc/palo-alto/palo-alto/palo-alto"
kind = "vendor"
title = "Palo Alto Networks NGFW"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "capture", "troubleshooting", "disk", "diagnostics", "ha", "interfaces", "jobs", "commit", "policy", "security", "sessions", "reboot", "system", "model-doc"]
risk = "high"
version = "PAN-OS 10.x/11.x"
---
The complete Palo Alto Networks NGFW document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Diag
<!-- section-tags: palo-alto, pan-os, cli, capture, troubleshooting, disk, diagnostics -->


### debug dataplane packet-diag (packet capture)

Captures packets on the dataplane using a packet-diag filter and capture toggle.

#### Command

```text
debug dataplane packet-diag set filter match source 10.1.1.10 destination 8.8.8.8
debug dataplane packet-diag set capture setting file cap1
debug dataplane packet-diag set capture on
debug dataplane packet-diag set capture off
```

#### Notes

- Set the filter first so you capture only the flow you need, then enable capture.
- Capture files land in /var/log/pan/pcap/ - pull them off with SCP from the management interface.
- Always clean up afterwards: `set capture off`, and clear the filter (`set filter match source 0`).
- Capture can impact dataplane performance - keep captures short and tightly filtered.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


### show system files

Lists files under /var/log with sizes: tech-support files, core dumps, pcaps.

#### Command

```text
show system files
```

#### Notes

- Quick check of what is consuming disk: old tech-support files and core dumps add up.
- Verify free space before software upgrades or taking new captures.
- Related view: `show system disk-space` for overall partition usage.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## HA
<!-- section-tags: palo-alto, pan-os, cli, ha -->


### request high-availability state sync

Triggers synchronization of configuration/state between the HA peers.

#### Command

```text
request high-availability state sync
```

#### Notes

- Use after resolving configuration drift between active and passive members.
- Can briefly impact the peer while it syncs; run it inside a maintenance window.
- Verify completion with `show high-availability state` (sync counters back to normal).
- Do not run concurrently with a commit or software upgrade.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


### show high-availability state

Reports HA mode and the local/peer state, including configuration and link sync status.

#### Command

```text
show high-availability state
```

#### Notes

- Shows local state (active/passive/suspended) and the peer's state in one view.
- First check after a failover or when you suspect split-brain.
- Config and session sync problems appear as out-of-sync warnings here.
- Peer unreachable or version mismatch shows up before any failover happens.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## Interfaces
<!-- section-tags: palo-alto, pan-os, cli, interfaces -->


### show interface all

Lists all interfaces with virtual router, zone, type and link state.

#### Command

```text
show interface all
show interface ethernet1/1
```

#### Notes

- State `up` is operational, `down` means no link, `n/a` means not configured or not attached.
- The name-filtered form shows IPs, counters and detail for a single interface.
- Verify each interface landed in the intended virtual router and zone.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## Jobs
<!-- section-tags: palo-alto, pan-os, cli, jobs, commit -->


### show jobs all

Lists commit, install and upgrade jobs with ID, status and progress.

#### Command

```text
show jobs all
show jobs id 25
```

#### Notes

- Confirm a commit or software install actually finished here before further changes.
- `show jobs id <n>` shows per-job detail including errors and the triggering user.
- Jobs stuck at a low percentage for a long time usually mean a pending reboot or hung process.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## Policy
<!-- section-tags: palo-alto, pan-os, cli, policy, security -->


### show running security-policy

Shows the security policy actually compiled and loaded into the running configuration.

#### Command

```text
show running security-policy
```

#### Notes

- Reflects the policy the dataplane enforces, including rules pushed by Panorama.
- Ends with the implicit `intrazone-default` and `interzone-default` rules.
- Compare against the candidate policy in the GUI when traffic does not behave after a commit.
- Use with `show session all` to see which running rule a flow matched.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## Sessions
<!-- section-tags: palo-alto, pan-os, cli, sessions, troubleshooting -->


### show session all

Lists active sessions with source, destination, application and the matching rule.

#### Command

```text
show session all
show session all filter source 10.1.1.10 destination 8.8.8.8
show session id 1234
```

#### Notes

- The filter form narrows output to the pair you care about - run `show session count` first on busy firewalls.
- `show session id <n>` gives full detail including NAT, QoS and packet counters.
- Check which security rule and egress interface a flow actually hit.
- Missing sessions mean the traffic was dropped before session setup or is not arriving.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


## System
<!-- section-tags: palo-alto, pan-os, cli, reboot, system -->


### request restart system

Reboots the firewall, taking management and data plane down until it returns.

#### Command

```text
request restart system
```

#### Notes

- Full outage for all traffic inspected by this device - schedule a maintenance window.
- In an HA pair, fail over to the peer first so the secondary reboots instead of the active.
- A dataplane-only restart also exists (`request restart dataplane`) but it drops traffic too.
- Confirm no commit or upgrade job is running with `show jobs all` before restarting.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com


### show system info

Shows software version, platform, serial number, uptime and management settings.

#### Command

```text
show system info
```

#### Notes

- The serial number and PAN-OS version are required for TAC cases.
- Also shows management IP, DNS/NTP servers and time of last commit.
- First command to run on an unfamiliar firewall to identify model and release.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com

