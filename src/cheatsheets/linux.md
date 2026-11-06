# Linux command cheat sheet

## Network interfaces & addresses

| Command | What it does |
|---|---|
| `ip a` | Show all interfaces with IP addresses |
| `ip -br a` | Brief interface/address list |
| `ip link set eth0 up` | Bring an interface up |
| `ip addr add 192.168.1.50/24 dev eth0` | Add an address |
| `ip route show` | Show the routing table |
| `ip route add 10.10.0.0/16 via 192.168.1.1` | Add a static route |
| `ip neigh` | Show the ARP/neighbor table |
| `ethtool eth0` | Link speed, duplex, driver info |

## Connectivity testing

| Command | What it does |
|---|---|
| `ping -c 4 8.8.8.8` | Four ICMP echoes |
| `ping -i 0.2 -c 100 192.168.1.1` | Fast ping, 5 pings/sec |
| `traceroute example.com` | Hop-by-hop path (ICMP/UDP) |
| `mtr 8.8.8.8` | Live combined ping + traceroute |
| `curl -I https://example.com` | HTTP HEAD, response headers |
| `nc -zv host 22` | TCP port check |
| `dig +short example.com` | Quick DNS A lookup |
| `dig @1.1.1.1 example.com MX` | Query a specific resolver, MX records |
| `host example.com` | Simple DNS lookup |
| `whois example.com` | Registry ownership info |

## Sockets & connections

| Command | What it does |
|---|---|
| `ss -tunlp` | Listening TCP/UDP sockets with owning process |
| `ss -tunap` | All sockets + processes (needs root for other users') |
| `ss -tn state established` | Established connections only |
| `ss -tn dport = :443` | Connections to port 443 |
| `lsof -i :80` | Which process holds port 80 |
| `lsof -i -n -P` | All network activity, numeric and fast |

## Capture & inspection

| Command | What it does |
|---|---|
| `tcpdump -i eth0 -n` | Sniff interface eth0, no DNS |
| `tcpdump -i eth0 host 192.168.1.1 and port 443` | Filter host and port |
| `tcpdump -w cap.pcap -c 1000` | Write 1000 packets to a file |
| `tcpdump -r cap.pcap -A` | Read back, show ASCII payloads |
| `nmap -sn 192.168.1.0/24` | Host discovery only (ping scan) |
| `nmap -sV -p 1-1000 192.168.1.10` | Version scan first 1000 ports |

## Firewall (nftables / iptables)

| Command | What it does |
|---|---|
| `nft list ruleset` | Dump the active nftables ruleset |
| `iptables -L -n -v` | Legacy filter table with counters |
| `iptables -A INPUT -p tcp --dport 22 -j ACCEPT` | Allow SSH |
| `ufw status verbose` | Ubuntu firewall status |
| `ufw allow from 192.168.1.0/24 to any port 22` | Scoped SSH allow |

## Files, processes & system

| Command | What it does |
|---|---|
| `journalctl -u nginx -f` | Follow a unit's logs |
| `journalctl --since "1 hour ago" -p err` | Errors from the last hour |
| `dmesg -T --level=err,warn` | Kernel ring buffer, human time |
| `df -h` / `du -sh *` | Disk usage: free space / per-folder |
| `htop` / `top` | Live process view |
| `ss -s` | Socket summary statistics |
| `vmstat 1` / `iostat 1` | Memory & IO per second |
| `lscpu` / `free -h` | CPU info / memory info |
| `tar -czf out.tgz dir/` | Compress a directory |
| `rsync -avP src/ dst/` | Resumable copy with progress |
| `chmod 600 key.pem` | Private key permissions |
| `grep -rn "pattern" /etc/` | Recursive text search |
| `find . -size +100M` | Files larger than 100 MB |

## SSH

| Command | What it does |
|---|---|
| `ssh -i key.pem user@host` | Connect with a key |
| `ssh -L 8080:localhost:80 user@host` | Local port forward |
| `ssh -R 9000:localhost:9000 user@host` | Remote port forward |
| `ssh -D 1080 user@host` | SOCKS proxy through SSH |
| `scp file user@host:/path/` | Copy a file over SSH |
| `ssh-keygen -t ed25519` | Generate a key pair |
| `ssh-copy-id user@host` | Install your public key |

## Performance quick probes

| Command | What it does |
|---|---|
| `curl -o /dev/null -s -w '%{time_total}\n' https://x.com` | Total request time |
| `ip -s link show eth0` | Interface RX/TX counters and errors |
| `nproc` | Core count |
| `uptime` | Load average over 1/5/15 min |
