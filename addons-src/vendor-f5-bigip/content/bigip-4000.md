---
id = "doc/f5/bigip/4000"
kind = "vendor"
title = "F5 BIG-IP 4000"
category = "reference/vendors/f5/bigip/4000"
tags = ["f5", "bigip", "tmsh", "model-doc"]
risk = "info"
version = "BIG-IP 16.1 / 17.x"
---
The complete BIG-IP 4000 model document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## System
<!-- section-tags: system, tmsh, platform -->


### show sys version

Shows the running BIG-IP software version, build, edition and platform model.

#### Command

```bash
tmsh show sys version
```

#### Notes

- First command on any tmsh session; confirm the version/build before applying known-issue fixes.
- The Edition line distinguishes point releases that carry specific bugfixes.
- On HA pairs run it on both units — version drift between peers blocks config sync.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### show sys hardware

Shows platform hardware details: chassis model, CPU, memory, and fan/temperature/power sensor readings.

#### Command

```bash
tmsh show sys hardware
```

#### Notes

- Sensor failures (fans, PSU, temperature) predict unplanned failovers — check this before and after maintenance windows.
- Confirms the exact platform model when ordering parts or validating license/platform compatibility.
- Read-only and safe to run at any time, including on active units.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### show sys performance

Shows system performance counters: CPU utilization, memory usage and uptime for the appliance.

#### Command

```bash
tmsh show sys performance
```

#### Notes

- Quick capacity check during slowness reports before diving into per-object statistics.
- Sustained high CPU with normal traffic usually means a heavy iRule, a profiling module, or logging overhead.
- Pair with show sys hardware to rule out physical problems (fans, temperature) behind the numbers.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## LTM — virtual servers
<!-- section-tags: ltm, virtual-servers, tmsh -->


### list ltm virtual

Prints the LTM virtual server configuration: destination, port, pool, profiles, iRules and SNAT settings.

#### Command

```bash
tmsh list ltm virtual
tmsh list ltm virtual /Common/vs-app-443
```

#### Notes

- The virtual server is the traffic entry point; verify destination/port and pool first when a service is down.
- Check the source-address-translation field here to see whether and how SNAT is applied.
- The profiles list (tcp, http, ssl client/server) explains most protocol-level behavior differences.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### show ltm virtual

Shows runtime statistics per virtual server: current connections, throughput and packet counters.

#### Command

```bash
tmsh show ltm virtual
tmsh show ltm virtual /Common/vs-app-443
```

#### Notes

- Zero current connections on a supposedly busy virtual points to monitor failures or upstream routing.
- Growing counters confirm traffic actually arrives; flat counters mean the path (or the VIP) is broken.
- Status (Availability/State) lines tell you whether the virtual itself is enabled and able to accept traffic.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## LTM — pools and monitors
<!-- section-tags: ltm, pools, monitors -->


### list ltm pool

Prints the pool configuration: members with their ports, load balancing method and attached health monitors.

#### Command

```bash
tmsh list ltm pool
tmsh list ltm pool /Common/pool-app
```

#### Notes

- Compare the configured members against reality: missing or wrong member IP/port is a classic 502 source.
- The load-balancing-method field (round-robin, least-connections-member) explains uneven distribution.
- Missing monitor entries means members are up regardless of real service health.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### show ltm pool

Shows runtime pool statistics and per-member status: availability (up/down), current connections and bytes.

#### Command

```bash
tmsh show ltm pool
tmsh show ltm pool /Common/pool-app members
```

#### Notes

- The members detail view is the go-to for member status: up (green), down, checking or disabled.
- Members flagged down by a monitor will not receive new connections even though the virtual is up.
- Combine with list ltm pool to separate configuration from runtime state when a member keeps flapping.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### list ltm monitor

Lists the health monitor definitions (http, https, tcp, gateway-icmp and more) with intervals, timeouts and send/receive strings.

#### Command

```bash
tmsh list ltm monitor
tmsh list ltm monitor http /Common/http-basic
```

#### Notes

- Interval and timeout decide how fast a dead member is pulled: interval 5 / timeout 16 is a common pattern.
- For HTTP monitors the receive-disable-string marks a member available but not in service (drain state).
- A receive string that never matches (wrong status code expected) keeps members down — test the probe manually.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## iRules
<!-- section-tags: ltm, irules, tcl -->


### list ltm rule (iRules)

Prints iRule definitions written in TCL: the events they hook and the logic applied to traffic.

#### Command

```bash
tmsh list ltm rule
tmsh list ltm rule /Common/ir-redirect-http
```

#### Notes

- iRules execute on events such as CLIENT_ACCEPTED, HTTP_REQUEST or HTTP_RESPONSE and can select pools, rewrite headers or redirect.
- Check which virtuals reference a rule before editing it — one rule often serves many applications.
- A buggy iRule on a hot path can stall all traffic on the virtual; test changes in a staging environment first.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### iRules basics

iRules are TCL scripts attached to virtual servers that inspect and steer traffic at defined processing events.

#### Structure

```tcl
when HTTP_REQUEST {
    if { [HTTP::host] eq "old.example.com" } {
        HTTP::redirect "https://www.example.com[HTTP::uri]"
    }
}
```

- Events: CLIENT_ACCEPTED (connection), HTTP_REQUEST/HTTP_RESPONSE (layer 7) are the most used.
- Commands: pool selects a pool, HTTP::redirect/HTTP::header rewrite requests and responses, IP::client_addr reads the source.

#### Notes

- Keep rules small and event-scoped; rules run per event on live traffic and cost CPU.
- Log with log local0. during development, then remove verbose logging before production.
- List and review with tmsh list ltm rule; prefer profiles and policy features when an iRule is only doing simple matching.

Source: F5 iRules and tmsh Reference Guide — https://techdocs.f5.com


## Connections
<!-- section-tags: connections, tmsh, troubleshooting -->


### show sys conn

Dumps the live connection table: client and server side endpoints, protocol and connection state.

#### Command

```bash
tmsh show sys conn
tmsh show sys conn | grep 10.10.10.5
```

#### Notes

- Shows client-side (cs) and server-side (ss) tuples so you can trace one flow end-to-end through the BIG-IP.
- Pipe through grep to filter for a client IP or virtual server; the full table is large on busy units.
- Stuck connections in an abnormal state point at idle timeouts, monitors, or a misconfigured SNAT causing asymmetry.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## HA — failover and sync
<!-- section-tags: ha, failover, sync -->


### show cm failover-status

Shows the HA failover state of the unit: active or standby, plus the failover color and reason.

#### Command

```bash
tmsh show cm failover-status
```

#### Notes

- Always confirm which unit is active before maintenance; run this on both peers.
- Color green with a stable state is healthy; amber/red states indicate network or peer problems.
- Pair with show cm sync-status to confirm both that the unit is active and the config is synchronized.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


### show cm sync-status

Shows the device group configuration sync status: whether both HA peers hold the same configuration.

#### Command

```bash
tmsh show cm sync-status
```

#### Notes

- In Sync / green is the healthy state; Changes Pending means one side has unsynced modifications.
- Fix sync issues before failover testing — an out-of-sync standby restores stale config when it becomes active.
- Details include the device group name and the last sync direction.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## Certificates
<!-- section-tags: certificates, ssl, tmsh -->


### install sys file ssl-cert

Imports an SSL certificate into the BIG-IP certificate store from a local file on the appliance.

#### Command

```bash
tmsh install sys file ssl-cert /Common/example.crt from-local-file /var/tmp/example.crt
```

#### Notes

- Copy the certificate to /var/tmp first (scp), then install it into the /Common partition.
- The certificate then appears in the store and can be attached to client-ssl profiles on virtual servers.
- Replacing an expired cert is straightforward; test the affected client-ssl profile afterwards in a browser.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com


## NAT and SNAT
<!-- section-tags: nat, snat -->


### SNAT and NAT concepts

On BIG-IP, source translation decides what source IP the pool member sees: none (client IP preserved), automap (self IP) or an SNAT pool.

#### Key pieces

- SNAT automap: translates the client source to the BIG-IP self IP, guaranteeing return traffic passes the unit.
- SNAT pool: translates to a dedicated NAT address range — needed when pools sit on networks without a route back via the BIG-IP.
- Fixed NAT (ltm nat): static one-to-one mappings, configured rarely compared to SNAT pools.
- Set per virtual server via source-address-translation (see tmsh list ltm virtual), with pools listed by tmsh list ltm snatpool.

#### Notes

- Skipping SNAT on asymmetric setups breaks connections because return traffic bypasses the BIG-IP.
- Automap hides client IPs from servers — log client IPs in X-Forwarded-For when servers need them.
- SNAT ports are finite: high-connection virtuals need adequate SNAT pool sizing.

Source: F5 BIG-IP LTM Concepts Guide — https://techdocs.f5.com


## Logs
<!-- section-tags: logs, logging -->


### Log files and logging

BIG-IP writes component logs to /var/log via syslogd; they are read over SSH (advanced shell) rather than through tmsh.

#### Key files

```bash
tail -f /var/log/ltm    # LTM events: monitor state changes, connection notices
tail -f /var/log/audit  # config changes via tmsh/GUI
tail -f /var/log/asm    # WAF (ASM) events
tail -f /var/log/gtm    # DNS/GTM events
```

#### Notes

- /var/log/ltm is the first place to look for monitor flaps and connection-level errors.
- For permanent retention, configure remote high-speed logging (HSL) or syslog destinations in the GUI.
- Log disk usage matters: archive or offload rather than letting /var fill up on the appliance.

Source: F5 BIG-IP System Logging Guide — https://techdocs.f5.com
