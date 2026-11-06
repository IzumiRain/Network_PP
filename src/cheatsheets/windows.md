# Windows cheat sheet — PowerShell & CMD

## Network configuration (PowerShell)

| Command | What it does |
|---|---|
| `Get-NetIPConfiguration` | Interfaces, addresses, gateways, DNS |
| `Get-NetIPAddress -AddressFamily IPv4` | All IPv4 addresses |
| `Get-NetAdapter \| ft Name,Status,LinkSpeed` | Adapter inventory |
| `New-NetIPAddress -InterfaceIndex 12 -IPAddress 192.168.1.50 -PrefixLength 24 -DefaultGateway 192.168.1.1` | Set a static IP |
| `Set-DnsClientServerAddress -InterfaceIndex 12 -ServerAddresses 1.1.1.1` | Set DNS servers |
| `Get-NetRoute -DestinationPrefix 0.0.0.0/0` | Default routes |
| `Test-NetConnection example.com -Port 443` | TCP connect test with diagnostics |
| `Test-NetConnection example.com -TraceRoute` | Traceroute with latency |
| `Resolve-DnsName example.com -Type MX` | DNS query (any record type) |
| `Get-NetNeighbor` | ARP neighbor table |
| `arp -a` | ARP table (CMD classic) |
| `Get-NetTCPConnection -State Established` | Live TCP table |
| `Get-NetTCPConnection -OwningProcess (Get-Process chrome).Id` | Connections per process |

## Network (CMD classic)

| Command | What it does |
|---|---|
| `ipconfig /all` | Full adapter configuration |
| `ipconfig /displaydns` | DNS resolver cache |
| `ipconfig /flushdns` | Clear DNS cache |
| `ipconfig /release` + `ipconfig /renew` | Reset DHCP lease |
| `ping -t 8.8.8.8` | Continuous ping (Ctrl+C to stop) |
| `ping -4 -n 5 host` | Force IPv4 |
| `tracert -d example.com` | Traceroute, no DNS |
| `pathping example.com` | Traceroute + per-hop loss stats |
| `nslookup example.com 1.1.1.1` | DNS query via a specific server |
| `netstat -ano` | All connections with PIDs |
| `netstat -ano \| findstr :443` | Who is on port 443 |
| `netstat -ab` | Connections with executable names (admin) |
| `route print` | Routing table |
| `netsh wlan show profiles` | Saved Wi-Fi networks |
| `netsh wlan show profile name="SSID" key=clear` | Show a stored Wi-Fi password |
| `netsh interface ip show config` | IP config per interface |

## Firewall

| Command | What it does |
|---|---|
| `Get-NetFirewallRule -Enabled True \| measure` | Count active rules |
| `New-NetFirewallRule -DisplayName "Allow SSH" -Direction Inbound -Protocol TCP -LocalPort 22 -Action Allow` | Add an inbound allow rule |
| `netsh advfirewall show allprofiles` | Firewall state per profile |
| `netsh advfirewall firewall add rule name="Web" dir=in action=allow protocol=TCP localport=8080` | Add rule (CMD) |

## Processes & services

| Command | What it does |
|---|---|
| `Get-Process \| sort CPU -Descending \| select -First 10` | Top 10 by CPU |
| `Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess` | Process owning a port |
| `Stop-Process -Id 4321 -Force` | Kill by PID |
| `Get-Service \| where Status -eq Running` | Running services |
| `Restart-Service Spooler` | Restart print spooler |
| `tasklist /svc /fi "PID eq 1234"` | Services inside a PID |
| `taskkill /PID 1234 /F` | Kill (CMD) |
| `sc query wuauserv` | Service detail |

## System info & events

| Command | What it does |
|---|---|
| `systeminfo` | Full system inventory |
| `Get-ComputerInfo -Property Os*,Cs*` | OS + chassis summary |
| `Get-EventLog -LogName System -Newest 20 -EntryType Error` | Recent system errors (classic) |
| `Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24)}` | Errors from last 24h (modern) |
| `wevtutil qe System /c:10 /f:text /q:"*[System[(Level=2)]]"` | Event query (CMD) |
| `Get-Volume` | Volumes and free space |
| `winver` | Windows version dialog |

## PowerShell essentials

| Command | What it does |
|---|---|
| `Get-Help Test-NetConnection -Examples` | Built-in help with examples |
| `Get-Command *firewall*` | Discover cmdlets by pattern |
| `Get-Member` | Inspect an object's properties/methods |
| `$env:PATH` | Read an environment variable |
| `1..254 \| %{ Test-Connection 192.168.1.$_ -Count 1 -Quiet }` | Quick /24 ping sweep (true/false) |
| `Set-ExecutionPolicy -Scope Process RemoteSigned` | Allow local scripts for this session |
| `Invoke-WebRequest example.com \| select -Expand Content` | Fetch a page |
| `Measure-Command { ping -n 1 8.8.8.8 }` | Time any command |

## Notes

- PowerShell aliases old CMD tools: `ping`, `tracert`, `netstat`, `nslookup` still work as executables.
- Most network cmdlets need an elevated PowerShell for changes; reads are usually fine as a normal user.
- `Test-NetConnection` is slower than a raw TCP connect — for bulk checks prefer a proper scanner.
