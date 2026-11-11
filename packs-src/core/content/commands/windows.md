---
id = "doc/os/windows/windows"
kind = "command"
title = "Windows Commands"
category = "reference/os/windows"
tags = ["windows", "network-configuration-powershell", "network-cmd-classic", "firewall", "processes-services", "system-info-events", "powershell-essentials", "notes", "model-doc"]
risk = "medium"
version = "core-0.3.0"
---
The complete Windows Commands document: every command for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from src/cheatsheets/windows.md; each topic is a
section carrying its own tags via a section-tags marker.


## Network configuration (PowerShell)
<!-- section-tags: windows, network-configuration-powershell -->

### Get-NetIPConfiguration

Interfaces, addresses, gateways, DNS

```powershell
Get-NetIPConfiguration
```

Risk: low

### Get-NetIPAddress -AddressFamily IPv4

All IPv4 addresses

```powershell
Get-NetIPAddress -AddressFamily IPv4
```

Risk: low

### Get-NetAdapter | ft Name,Status,LinkSpeed

Adapter inventory

```powershell
Get-NetAdapter | ft Name,Status,LinkSpeed
```

Risk: low

### New-NetIPAddress -InterfaceIndex 12 -IPAddress 192.168.1.50 -PrefixLength 24 -DefaultGateway 192.168.1.1

Set a static IP

```powershell
New-NetIPAddress -InterfaceIndex 12 -IPAddress 192.168.1.50 -PrefixLength 24 -DefaultGateway 192.168.1.1
```

Risk: low

### Set-DnsClientServerAddress -InterfaceIndex 12 -ServerAddresses 1.1.1.1

Set DNS servers

```powershell
Set-DnsClientServerAddress -InterfaceIndex 12 -ServerAddresses 1.1.1.1
```

Risk: low

### Get-NetRoute -DestinationPrefix 0.0.0.0/0

Default routes

```powershell
Get-NetRoute -DestinationPrefix 0.0.0.0/0
```

Risk: low

### Test-NetConnection example.com -Port 443

TCP connect test with diagnostics

```powershell
Test-NetConnection example.com -Port 443
```

Risk: low

### Test-NetConnection example.com -TraceRoute

Traceroute with latency

```powershell
Test-NetConnection example.com -TraceRoute
```

Risk: low

### Resolve-DnsName example.com -Type MX

DNS query (any record type)

```powershell
Resolve-DnsName example.com -Type MX
```

Risk: low

### Get-NetNeighbor

ARP neighbor table

```powershell
Get-NetNeighbor
```

Risk: low

### arp -a

ARP table (CMD classic)

```powershell
arp -a
```

Risk: low

### Get-NetTCPConnection -State Established

Live TCP table

```powershell
Get-NetTCPConnection -State Established
```

Risk: low

### Get-NetTCPConnection -OwningProcess (Get-Process chrome).Id

Connections per process

```powershell
Get-NetTCPConnection -OwningProcess (Get-Process chrome).Id
```

Risk: low

## Network (CMD classic)
<!-- section-tags: windows, network-cmd-classic -->

### ipconfig /all

Full adapter configuration

```powershell
ipconfig /all
```

Risk: low

### ipconfig /displaydns

DNS resolver cache

```powershell
ipconfig /displaydns
```

Risk: low

### ipconfig /flushdns

Clear DNS cache

```powershell
ipconfig /flushdns
```

Risk: low

### ipconfig /release + ipconfig /renew

Reset DHCP lease

```powershell
ipconfig /release + ipconfig /renew
```

Risk: medium

### ping -t 8.8.8.8

Continuous ping (Ctrl+C to stop)

```powershell
ping -t 8.8.8.8
```

Risk: low

### ping -4 -n 5 host

Force IPv4

```powershell
ping -4 -n 5 host
```

Risk: low

### tracert -d example.com

Traceroute, no DNS

```powershell
tracert -d example.com
```

Risk: low

### pathping example.com

Traceroute + per-hop loss stats

```powershell
pathping example.com
```

Risk: low

### nslookup example.com 1.1.1.1

DNS query via a specific server

```powershell
nslookup example.com 1.1.1.1
```

Risk: low

### netstat -ano

All connections with PIDs

```powershell
netstat -ano
```

Risk: low

### netstat -ano | findstr :443

Who is on port 443

```powershell
netstat -ano | findstr :443
```

Risk: low

### netstat -ab

Connections with executable names (admin)

```powershell
netstat -ab
```

Risk: low

### route print

Routing table

```powershell
route print
```

Risk: low

### netsh wlan show profiles

Saved Wi-Fi networks

```powershell
netsh wlan show profiles
```

Risk: low

### netsh wlan show profile name="SSID" key=clear

Show a stored Wi-Fi password

```powershell
netsh wlan show profile name="SSID" key=clear
```

Risk: low

### netsh interface ip show config

IP config per interface

```powershell
netsh interface ip show config
```

Risk: low

## Firewall
<!-- section-tags: windows, firewall -->

### Get-NetFirewallRule -Enabled True | measure

Count active rules

```powershell
Get-NetFirewallRule -Enabled True | measure
```

Risk: low

### New-NetFirewallRule -DisplayName "Allow SSH" -Direction Inbound -Protocol TCP -LocalPort 22 -Action Allow

Add an inbound allow rule

```powershell
New-NetFirewallRule -DisplayName "Allow SSH" -Direction Inbound -Protocol TCP -LocalPort 22 -Action Allow
```

Risk: low

### netsh advfirewall show allprofiles

Firewall state per profile

```powershell
netsh advfirewall show allprofiles
```

Risk: low

### netsh advfirewall firewall add rule name="Web" dir=in action=allow protocol=TCP localport=8080

Add rule (CMD)

```powershell
netsh advfirewall firewall add rule name="Web" dir=in action=allow protocol=TCP localport=8080
```

Risk: low

## Processes & services
<!-- section-tags: windows, processes-services -->

### Get-Process | sort CPU -Descending | select -First 10

Top 10 by CPU

```powershell
Get-Process | sort CPU -Descending | select -First 10
```

Risk: low

### Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess

Process owning a port

```powershell
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080).OwningProcess
```

Risk: low

### Stop-Process -Id 4321 -Force

Kill by PID

```powershell
Stop-Process -Id 4321 -Force
```

Risk: low

### Get-Service | where Status -eq Running

Running services

```powershell
Get-Service | where Status -eq Running
```

Risk: low

### Restart-Service Spooler

Restart print spooler

```powershell
Restart-Service Spooler
```

Risk: low

### tasklist /svc /fi "PID eq 1234"

Services inside a PID

```powershell
tasklist /svc /fi "PID eq 1234"
```

Risk: low

### taskkill /PID 1234 /F

Kill (CMD)

```powershell
taskkill /PID 1234 /F
```

Risk: low

### sc query wuauserv

Service detail

```powershell
sc query wuauserv
```

Risk: low

## System info & events
<!-- section-tags: windows, system-info-events -->

### systeminfo

Full system inventory

```powershell
systeminfo
```

Risk: low

### Get-ComputerInfo -Property Os*,Cs*

OS + chassis summary

```powershell
Get-ComputerInfo -Property Os*,Cs*
```

Risk: low

### Get-EventLog -LogName System -Newest 20 -EntryType Error

Recent system errors (classic)

```powershell
Get-EventLog -LogName System -Newest 20 -EntryType Error
```

Risk: low

### Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24)}

Errors from last 24h (modern)

```powershell
Get-WinEvent -FilterHashtable @{LogName='System';Level=2;StartTime=(Get-Date).AddHours(-24)}
```

Risk: low

### wevtutil qe System /c:10 /f:text /q:"*[System[(Level=2)]]"

Event query (CMD)

```powershell
wevtutil qe System /c:10 /f:text /q:"*[System[(Level=2)]]"
```

Risk: low

### Get-Volume

Volumes and free space

```powershell
Get-Volume
```

Risk: low

### winver

Windows version dialog

```powershell
winver
```

Risk: low

## PowerShell essentials
<!-- section-tags: windows, powershell-essentials -->

### Get-Help Test-NetConnection -Examples

Built-in help with examples

```powershell
Get-Help Test-NetConnection -Examples
```

Risk: low

### Get-Command *firewall*

Discover cmdlets by pattern

```powershell
Get-Command *firewall*
```

Risk: low

### Get-Member

Inspect an object's properties/methods

```powershell
Get-Member
```

Risk: low

### $env:PATH

Read an environment variable

```powershell
$env:PATH
```

Risk: low

### 1..254 | %{ Test-Connection 192.168.1.$_ -Count 1 -Quiet }

Quick /24 ping sweep (true/false)

```powershell
1..254 | %{ Test-Connection 192.168.1.$_ -Count 1 -Quiet }
```

Risk: low

### Set-ExecutionPolicy -Scope Process RemoteSigned

Allow local scripts for this session

```powershell
Set-ExecutionPolicy -Scope Process RemoteSigned
```

Risk: low

### Invoke-WebRequest example.com | select -Expand Content

Fetch a page

```powershell
Invoke-WebRequest example.com | select -Expand Content
```

Risk: low

### Measure-Command { ping -n 1 8.8.8.8 }

Time any command

```powershell
Measure-Command { ping -n 1 8.8.8.8 }
```

Risk: low

## Notes
<!-- section-tags: windows, notes -->

- PowerShell aliases old CMD tools: `ping`, `tracert`, `netstat`, `nslookup` still work as executables.
- Most network cmdlets need an elevated PowerShell for changes; reads are usually fine as a normal user.
- `Test-NetConnection` is slower than a raw TCP connect — for bulk checks prefer a proper scanner.
