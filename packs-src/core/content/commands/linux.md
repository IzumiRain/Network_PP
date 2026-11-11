---
id = "doc/os/linux/linux"
kind = "command"
title = "Linux Commands"
category = "reference/os/linux"
tags = ["linux", "network-interfaces-addresses", "connectivity-testing", "sockets-connections", "capture-inspection", "firewall-nftables-iptables", "files-processes-system", "ssh", "performance-quick-probes", "model-doc"]
risk = "low"
version = "core-0.3.0"
---
The complete Linux Commands document: every command for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from src/cheatsheets/linux.md; each topic is a
section carrying its own tags via a section-tags marker.


## Network interfaces & addresses
<!-- section-tags: linux, network-interfaces-addresses -->

### ip a

Show all interfaces with IP addresses

```bash
ip a
```

Risk: low

### ip -br a

Brief interface/address list

```bash
ip -br a
```

Risk: low

### ip link set eth0 up

Bring an interface up

```bash
ip link set eth0 up
```

Risk: low

### ip addr add 192.168.1.50/24 dev eth0

Add an address

```bash
ip addr add 192.168.1.50/24 dev eth0
```

Risk: low

### ip route show

Show the routing table

```bash
ip route show
```

Risk: low

### ip route add 10.10.0.0/16 via 192.168.1.1

Add a static route

```bash
ip route add 10.10.0.0/16 via 192.168.1.1
```

Risk: low

### ip neigh

Show the ARP/neighbor table

```bash
ip neigh
```

Risk: low

### ethtool eth0

Link speed, duplex, driver info

```bash
ethtool eth0
```

Risk: low

## Connectivity testing
<!-- section-tags: linux, connectivity-testing -->

### ping -c 4 8.8.8.8

Four ICMP echoes

```bash
ping -c 4 8.8.8.8
```

Risk: low

### ping -i 0.2 -c 100 192.168.1.1

Fast ping, 5 pings/sec

```bash
ping -i 0.2 -c 100 192.168.1.1
```

Risk: low

### traceroute example.com

Hop-by-hop path (ICMP/UDP)

```bash
traceroute example.com
```

Risk: low

### mtr 8.8.8.8

Live combined ping + traceroute

```bash
mtr 8.8.8.8
```

Risk: low

### curl -I https://example.com

HTTP HEAD, response headers

```bash
curl -I https://example.com
```

Risk: low

### nc -zv host 22

TCP port check

```bash
nc -zv host 22
```

Risk: low

### dig +short example.com

Quick DNS A lookup

```bash
dig +short example.com
```

Risk: low

### dig @1.1.1.1 example.com MX

Query a specific resolver, MX records

```bash
dig @1.1.1.1 example.com MX
```

Risk: low

### host example.com

Simple DNS lookup

```bash
host example.com
```

Risk: low

### whois example.com

Registry ownership info

```bash
whois example.com
```

Risk: low

## Sockets & connections
<!-- section-tags: linux, sockets-connections -->

### ss -tunlp

Listening TCP/UDP sockets with owning process

```bash
ss -tunlp
```

Risk: low

### ss -tunap

All sockets + processes (needs root for other users')

```bash
ss -tunap
```

Risk: low

### ss -tn state established

Established connections only

```bash
ss -tn state established
```

Risk: low

### ss -tn dport = :443

Connections to port 443

```bash
ss -tn dport = :443
```

Risk: low

### lsof -i :80

Which process holds port 80

```bash
lsof -i :80
```

Risk: low

### lsof -i -n -P

All network activity, numeric and fast

```bash
lsof -i -n -P
```

Risk: low

## Capture & inspection
<!-- section-tags: linux, capture-inspection -->

### tcpdump -i eth0 -n

Sniff interface eth0, no DNS

```bash
tcpdump -i eth0 -n
```

Risk: low

### tcpdump -i eth0 host 192.168.1.1 and port 443

Filter host and port

```bash
tcpdump -i eth0 host 192.168.1.1 and port 443
```

Risk: low

### tcpdump -w cap.pcap -c 1000

Write 1000 packets to a file

```bash
tcpdump -w cap.pcap -c 1000
```

Risk: low

### tcpdump -r cap.pcap -A

Read back, show ASCII payloads

```bash
tcpdump -r cap.pcap -A
```

Risk: low

### nmap -sn 192.168.1.0/24

Host discovery only (ping scan)

```bash
nmap -sn 192.168.1.0/24
```

Risk: low

### nmap -sV -p 1-1000 192.168.1.10

Version scan first 1000 ports

```bash
nmap -sV -p 1-1000 192.168.1.10
```

Risk: low

## Firewall (nftables / iptables)
<!-- section-tags: linux, firewall-nftables-iptables -->

### nft list ruleset

Dump the active nftables ruleset

```bash
nft list ruleset
```

Risk: low

### iptables -L -n -v

Legacy filter table with counters

```bash
iptables -L -n -v
```

Risk: low

### iptables -A INPUT -p tcp --dport 22 -j ACCEPT

Allow SSH

```bash
iptables -A INPUT -p tcp --dport 22 -j ACCEPT
```

Risk: low

### ufw status verbose

Ubuntu firewall status

```bash
ufw status verbose
```

Risk: low

### ufw allow from 192.168.1.0/24 to any port 22

Scoped SSH allow

```bash
ufw allow from 192.168.1.0/24 to any port 22
```

Risk: low

## Files, processes & system
<!-- section-tags: linux, files-processes-system -->

### journalctl -u nginx -f

Follow a unit's logs

```bash
journalctl -u nginx -f
```

Risk: low

### journalctl --since "1 hour ago" -p err

Errors from the last hour

```bash
journalctl --since "1 hour ago" -p err
```

Risk: low

### dmesg -T --level=err,warn

Kernel ring buffer, human time

```bash
dmesg -T --level=err,warn
```

Risk: low

### df -h / du -sh *

Disk usage: free space / per-folder

```bash
df -h / du -sh *
```

Risk: low

### htop / top

Live process view

```bash
htop / top
```

Risk: low

### ss -s

Socket summary statistics

```bash
ss -s
```

Risk: low

### vmstat 1 / iostat 1

Memory & IO per second

```bash
vmstat 1 / iostat 1
```

Risk: low

### lscpu / free -h

CPU info / memory info

```bash
lscpu / free -h
```

Risk: low

### tar -czf out.tgz dir/

Compress a directory

```bash
tar -czf out.tgz dir/
```

Risk: low

### rsync -avP src/ dst/

Resumable copy with progress

```bash
rsync -avP src/ dst/
```

Risk: low

### chmod 600 key.pem

Private key permissions

```bash
chmod 600 key.pem
```

Risk: low

### grep -rn "pattern" /etc/

Recursive text search

```bash
grep -rn "pattern" /etc/
```

Risk: low

### find . -size +100M

Files larger than 100 MB

```bash
find . -size +100M
```

Risk: low

## SSH
<!-- section-tags: linux, ssh -->

### ssh -i key.pem user@host

Connect with a key

```bash
ssh -i key.pem user@host
```

Risk: low

### ssh -L 8080:localhost:80 user@host

Local port forward

```bash
ssh -L 8080:localhost:80 user@host
```

Risk: low

### ssh -R 9000:localhost:9000 user@host

Remote port forward

```bash
ssh -R 9000:localhost:9000 user@host
```

Risk: low

### ssh -D 1080 user@host

SOCKS proxy through SSH

```bash
ssh -D 1080 user@host
```

Risk: low

### scp file user@host:/path/

Copy a file over SSH

```bash
scp file user@host:/path/
```

Risk: low

### ssh-keygen -t ed25519

Generate a key pair

```bash
ssh-keygen -t ed25519
```

Risk: low

### ssh-copy-id user@host

Install your public key

```bash
ssh-copy-id user@host
```

Risk: low

## Performance quick probes
<!-- section-tags: linux, performance-quick-probes -->

### curl -o /dev/null -s -w '%{time_total}\n' https://x.com

Total request time

```bash
curl -o /dev/null -s -w '%{time_total}\n' https://x.com
```

Risk: low

### ip -s link show eth0

Interface RX/TX counters and errors

```bash
ip -s link show eth0
```

Risk: low

### nproc

Core count

```bash
nproc
```

Risk: low

### uptime

Load average over 1/5/15 min

```bash
uptime
```

Risk: low
