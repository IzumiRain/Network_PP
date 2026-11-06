use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::time::{Duration, Instant};

use super::cidr;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortScanResult {
    pub port: u16,
    pub state: PortState,
    pub service: String,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub latency_ms: Option<u64>,
    pub open_ports: Vec<u16>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PingResult {
    pub ip: String,
    pub up: bool,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArpEntry {
    pub ip: String,
    pub mac: String,
    pub vendor: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocalNetworkInfo {
    pub local_ip: String,
    pub suggested_cidr: String,
}

const MAX_HOSTS: u64 = 65_536;
const MAX_PORTS_PER_SCAN: usize = 2_048;
const HTTP_PROBE_PORTS: &[u16] = &[80, 8080, 8000, 8008, 8888];

/// Parse "80,443,1000-1010" into a sorted, de-duplicated port list.
pub fn parse_port_spec(spec: &str) -> Result<Vec<u16>, String> {
    let mut out = std::collections::BTreeSet::new();
    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((a, b)) = part.split_once('-') {
            let a: u16 = a
                .trim()
                .parse()
                .map_err(|_| format!("Invalid port '{a}' in range"))?;
            let b: u16 = b
                .trim()
                .parse()
                .map_err(|_| format!("Invalid port '{b}' in range"))?;
            if b < a {
                return Err(format!("Range {part} is backwards"));
            }
            for p in a..=b {
                out.insert(p);
            }
        } else {
            let p: u16 = part.parse().map_err(|_| format!("Invalid port '{part}'"))?;
            out.insert(p);
        }
    }
    if out.is_empty() {
        return Err("No ports given — try e.g. 22,80,443".into());
    }
    if out.len() > MAX_PORTS_PER_SCAN {
        return Err(format!(
            "{} ports requested — limit is {MAX_PORTS_PER_SCAN} per scan",
            out.len()
        ));
    }
    Ok(out.into_iter().collect())
}

pub fn service_name(port: u16) -> String {
    super::ports::lookup(port as u32)
        .ok()
        .and_then(|r| r.assignments.into_iter().next())
        .map(|a| a.service.to_string())
        .unwrap_or_else(|| "unknown".into())
}

fn grab_banner(stream: &mut TcpStream, port: u16) -> Option<String> {
    stream.set_read_timeout(Some(Duration::from_millis(400))).ok()?;
    let mut buf = [0u8; 256];
    let read = std::io::Read::read(stream, &mut buf);
    let n = match read {
        Ok(n) => n,
        Err(_) => {
            if !HTTP_PROBE_PORTS.contains(&port) {
                return None;
            }
            let probe = b"HEAD / HTTP/1.0\r\nUser-Agent: Network++\r\n\r\n";
            std::io::Write::write_all(stream, probe).ok()?;
            stream.set_read_timeout(Some(Duration::from_millis(600))).ok()?;
            std::io::Read::read(stream, &mut buf).ok()?
        }
    };
    let text = String::from_utf8_lossy(&buf[..n]);
    text.lines()
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn tcp_scan_port(host: &str, port: u16, timeout_ms: u64, with_banner: bool) -> PortScanResult {
    let mut state = PortState::Filtered;
    let mut banner = None;

    let addrs: Vec<IpAddr> = format!("{host}:{port}")
        .to_socket_addrs()
        .map(|it| it.map(|a| a.ip()).collect())
        .unwrap_or_default();

    for a in &addrs {
        let sock = SocketAddr::new(*a, port);
        match TcpStream::connect_timeout(&sock, Duration::from_millis(timeout_ms)) {
            Ok(mut s) => {
                state = PortState::Open;
                if with_banner {
                    banner = grab_banner(&mut s, port);
                }
                break;
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::ConnectionRefused => {
                    state = PortState::Closed;
                    break;
                }
                _ => continue,
            },
        }
    }

    PortScanResult {
        port,
        state,
        service: service_name(port),
        banner,
    }
}

pub fn tcp_scan(
    host: &str,
    port_spec: &str,
    timeout_ms: u64,
    with_banner: bool,
    max_threads: usize,
) -> Result<Vec<PortScanResult>, String> {
    let ports = parse_port_spec(port_spec)?;
    let workers = max_threads.clamp(1, 512);
    let ports_ref = &ports;
    let host_ref = host;
    let mut results: Vec<PortScanResult> = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in ports_ref.chunks((ports_ref.len() / workers).max(1)) {
            handles.push(scope.spawn(move || {
                chunk
                    .iter()
                    .map(|&p| tcp_scan_port(host_ref, p, timeout_ms, with_banner))
                    .collect::<Vec<_>>()
            }));
        }
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect::<Vec<_>>()
    });
    results.sort_by_key(|r| r.port);
    Ok(results)
}

fn expand_cidr_hosts(input: &str) -> Result<Vec<Ipv4Addr>, String> {
    let info = cidr::calculate(input)?;
    if info.total_addresses > MAX_HOSTS {
        return Err(format!(
            "{} addresses — scanning is capped at {MAX_HOSTS}. Use a smaller range.",
            info.total_addresses
        ));
    }
    let start = info.network_int as u32;
    let mut out = Vec::with_capacity(info.total_addresses as usize);
    for i in 0..info.total_addresses as u32 {
        out.push(Ipv4Addr::from(start + i));
    }
    Ok(out)
}

pub fn discover_hosts(
    cidr_input: &str,
    port_spec: &str,
    timeout_ms: u64,
    max_threads: usize,
) -> Result<Vec<HostResult>, String> {
    let ports = parse_port_spec(port_spec)?;
    let hosts = expand_cidr_hosts(cidr_input)?;
    let workers = max_threads.clamp(1, 256);
    let ports_ref = &ports;
    let mut results: Vec<HostResult> = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in hosts.chunks((hosts.len() / workers).max(1)) {
            handles.push(scope.spawn(move || {
                chunk
                    .iter()
                    .filter_map(|ip| {
                        let ip_s = ip.to_string();
                        let t0 = Instant::now();
                        let mut open = Vec::new();
                        for &p in ports_ref.iter() {
                            if tcp_scan_port(&ip_s, p, timeout_ms, false).state == PortState::Open {
                                open.push(p);
                            }
                        }
                        if open.is_empty() {
                            return None;
                        }
                        let hostname = reverse_dns(&ip_s);
                        Some(HostResult {
                            ip: ip_s,
                            hostname,
                            latency_ms: Some(t0.elapsed().as_millis() as u64),
                            open_ports: open,
                        })
                    })
                    .collect::<Vec<_>>()
            }));
        }
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect::<Vec<_>>()
    });
    results.sort_by(|a, b| a.ip.cmp(&b.ip));
    Ok(results)
}

/// Reverse DNS via system resolvers — std has no reverse lookup.
pub fn reverse_dns(ip: &str) -> Option<String> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        ("nslookup", vec!["-timeout=1", ip])
    } else if cfg!(target_os = "macos") {
        ("host", vec!["-W", "1", ip])
    } else {
        ("getent", vec!["hosts", ip])
    };
    let output = std::process::Command::new(cmd).args(&args).output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let name = if cfg!(target_os = "windows") {
        text.lines()
            .find(|l| l.to_lowercase().starts_with("name:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.trim_end_matches('.').to_string())
    } else if cfg!(target_os = "macos") {
        text.lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.trim_end_matches('.').to_string())
    } else {
        text.lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.to_string())
    }?;
    if name == ip {
        None
    } else {
        Some(name)
    }
}

pub fn ping_sweep(
    cidr_input: &str,
    timeout_ms: u64,
    max_threads: usize,
) -> Result<Vec<PingResult>, String> {
    let hosts = expand_cidr_hosts(cidr_input)?;
    let workers = max_threads.clamp(1, 128);
    let results: Vec<PingResult> = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in hosts.chunks((hosts.len() / workers).max(1)) {
            handles.push(scope.spawn(move || {
                chunk
                    .iter()
                    .map(|ip| ping_one(&ip.to_string(), timeout_ms))
                    .collect::<Vec<_>>()
            }));
        }
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect::<Vec<_>>()
    });
    Ok(results)
}

pub fn ping_one(ip: &str, timeout_ms: u64) -> PingResult {
    let (up, latency) = system_ping(ip, timeout_ms);
    PingResult {
        ip: ip.to_string(),
        up,
        latency_ms: latency,
    }
}

fn system_ping(ip: &str, timeout_ms: u64) -> (bool, Option<u64>) {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("ping")
            .args(["-n", "1", "-w", &timeout_ms.to_string(), ip])
            .output()
    } else {
        let secs = ((timeout_ms as f64 / 1000.0).ceil() as u64).max(1);
        std::process::Command::new("ping")
            .args(["-c", "1", "-W", &secs.to_string(), ip])
            .output()
    };
    match output {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout).to_lowercase();
            let up = text.contains("ttl=") || text.contains("time<") || text.contains("time=");
            let latency = extract_ping_latency(&text);
            (up, latency)
        }
        Err(_) => (false, None),
    }
}

fn extract_ping_latency(text: &str) -> Option<u64> {
    if let Some(i) = text.find("time<") {
        let rest = &text[i + 5..];
        let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        return num.parse::<u64>().ok().map(|v| v.max(1));
    }
    let i = text.find("time=")?;
    let rest = &text[i + 5..];
    let num: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    num.parse::<f64>().ok().map(|v| v.round() as u64)
}

pub fn arp_table() -> Result<Vec<ArpEntry>, String> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        ("arp", vec!["-a"])
    } else if cfg!(target_os = "macos") {
        ("arp", vec!["-an"])
    } else {
        ("ip", vec!["neigh", "show"])
    };
    let output = std::process::Command::new(cmd)
        .args(&args)
        .output()
        .map_err(|e| format!("Could not run {cmd}: {e}"))?;
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(parse_arp_output(&text, cmd == "arp" && cfg!(target_os = "windows")))
}

pub fn parse_arp_output(text: &str, windows_format: bool) -> Vec<ArpEntry> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("Interface") || line.contains("internet address") {
            continue;
        }
        if windows_format {
            // e.g. "192.168.1.1          11-22-33-44-55-66     dynamic"
            let mut it = line.split_whitespace();
            let ip = it.next().unwrap_or_default();
            let mac = it.next().unwrap_or_default();
            let state = it.next().unwrap_or("dynamic").to_string();
            if cidr::parse_ipv4(ip).is_some() && mac.contains('-') {
                let mac_norm = mac.replace('-', ":").to_lowercase();
                out.push(ArpEntry {
                    ip: ip.to_string(),
                    mac: mac_norm.clone(),
                    vendor: oui_vendor(&mac_norm),
                    state,
                });
            }
        } else {
            // ip neigh: "192.168.1.1 dev eth0 lladdr 11:22:33:44:55:66 REACHABLE"
            // arp -an: "? (192.168.1.1) at 0:1f:33:aa:bb:cc on en0 ifscope [ethernet]"
            let first = line.split_whitespace().next().unwrap_or_default();
            let ip = if cidr::parse_ipv4(first).is_some() {
                first.to_string()
            } else if let Some(inner) = line.split('(').nth(1).and_then(|s| s.split(')').next()) {
                inner.to_string()
            } else {
                continue;
            };
            if cidr::parse_ipv4(&ip).is_none() {
                continue;
            }
            let mac = line
                .split("lladdr ")
                .nth(1)
                .or_else(|| line.split(" at ").nth(1))
                .map(|s| {
                    s.split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_lowercase()
                })
                .unwrap_or_default();
            if mac.is_empty() || mac == "<incomplete>" {
                continue;
            }
            let mac = normalize_mac(&mac);
            let state = if line.contains("REACHABLE") {
                "reachable"
            } else if line.contains("STALE") {
                "stale"
            } else if line.contains("FAILED") {
                "failed"
            } else {
                "known"
            };
            out.push(ArpEntry {
                ip,
                mac: mac.clone(),
                vendor: oui_vendor(&mac),
                state: state.to_string(),
            });
        }
    }
    out
}

/// Pad colon-separated hex groups to two digits: "0:1f:33" -> "00:1f:33".
fn normalize_mac(mac: &str) -> String {
    mac.split(':')
        .map(|g| format!("{:0>2}", g))
        .collect::<Vec<_>>()
        .join(":")
}

/// Curated OUI prefixes — only entries with well-established, widely-known owners.
const OUI_DB: &[(&str, &str)] = &[
    ("00:00:0c", "Cisco"),
    ("00:05:69", "VMware"),
    ("00:0c:29", "VMware"),
    ("00:1c:14", "VMware"),
    ("00:50:56", "VMware"),
    ("00:15:5d", "Microsoft Hyper-V"),
    ("00:03:93", "Apple"),
    ("ac:de:48", "Apple"),
    ("00:17:88", "Philips"),
    ("b8:27:eb", "Raspberry Pi"),
    ("dc:a6:32", "Raspberry Pi"),
    ("e4:5f:01", "Raspberry Pi"),
    ("00:e0:4c", "Realtek"),
    ("52:54:00", "QEMU/KVM"),
    ("00:16:3e", "Xen"),
    ("00:1a:11", "Google"),
    ("3c:5a:b4", "Google"),
    ("f4:f5:26", "Google"),
    ("54:60:09", "Google"),
    ("fc:f1:52", "Samsung"),
    ("00:16:32", "Samsung"),
    ("84:38:38", "Huawei"),
    ("00:18:82", "Huawei"),
    ("00:25:9e", "Huawei"),
    ("00:1e:10", "Hikvision"),
    ("44:47:cc", "Samsung"),
    ("00:26:37", "Micro-Star"),
    ("00:d8:61", "ASUSTek"),
    ("04:d9:f5", "ASUSTek"),
    ("00:1b:fc", "Hewlett Packard"),
    ("3c:d9:2b", "Hewlett Packard"),
    ("08:94:1a", "Apple"),
    ("f0:18:98", "Apple"),
    ("a4:5e:60", "Apple"),
    ("d0:03:4b", "Apple"),
    ("2c:cf:67", "Espressif"),
    ("24:6f:28", "Espressif"),
    ("30:ae:a4", "Espressif"),
    ("00:1f:33", "Netgear"),
    ("9c:3d:cf", "Netgear"),
    ("00:18:4d", "Netgear"),
    ("c8:3a:35", "Tenda"),
    ("50:2b:73", "TP-Link"),
    ("00:27:19", "ASRock"),
    ("0c:9d:21", "AzureWave"),
    ("00:0d:3a", "Microsoft Azure"),
    ("42:01:00", "Google Cloud"),
    ("06:00:00", "Amazon/AWS"),
];

pub fn oui_vendor(mac: &str) -> String {
    let prefix: String = mac.to_lowercase().chars().take(8).collect();
    for (oui, vendor) in OUI_DB {
        if prefix == *oui {
            return vendor.to_string();
        }
    }
    "Unknown vendor".into()
}

pub fn local_network_info() -> Result<LocalNetworkInfo, String> {
    // UDP connect trick: picks the outbound interface without sending packets.
    let sock = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    sock.connect("8.8.8.8:53").map_err(|e| e.to_string())?;
    let ip = sock.local_addr().map_err(|e| e.to_string())?.ip();
    match ip {
        IpAddr::V4(v4) => {
            let octets = v4.octets();
            Ok(LocalNetworkInfo {
                local_ip: v4.to_string(),
                suggested_cidr: format!("{}.{}.{}.{}/24", octets[0], octets[1], octets[2], 0),
            })
        }
        IpAddr::V6(_) => Err("No IPv4 interface found — local scanning needs IPv4".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_port_spec() {
        assert_eq!(parse_port_spec("80,443").unwrap(), vec![80, 443]);
        assert_eq!(
            parse_port_spec("1000-1004").unwrap(),
            vec![1000, 1001, 1002, 1003, 1004]
        );
        assert_eq!(parse_port_spec("443, 80 ,80").unwrap(), vec![80, 443]);
        assert!(parse_port_spec("").is_err());
        assert!(parse_port_spec("1010-1000").is_err());
        assert!(parse_port_spec("abc").is_err());
        assert!(parse_port_spec("70000").is_err());
        assert!(parse_port_spec("1-65535").is_err()); // over scan cap
    }

    #[test]
    fn caps_host_expansion() {
        assert!(expand_cidr_hosts("10.0.0.0/8").is_err());
        assert_eq!(expand_cidr_hosts("192.168.1.248/29").unwrap().len(), 8);
        assert_eq!(
            expand_cidr_hosts("192.168.1.1/32").unwrap()[0].to_string(),
            "192.168.1.1"
        );
    }

    #[test]
    fn parses_windows_arp() {
        let text = "\nInterface: 192.168.1.10 --- 0xd\n  Internet Address      Physical Address      Type\n  192.168.1.1          11-22-33-44-55-66     dynamic\n  192.168.1.50         00-00-0c-9f-f0-01     dynamic\n  224.0.0.22           01-00-5e-00-00-16     static\n";
        let rows = parse_arp_output(text, true);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].ip, "192.168.1.1");
        assert_eq!(rows[0].mac, "11:22:33:44:55:66");
        assert_eq!(rows[0].state, "dynamic");
        assert_eq!(rows[1].vendor, "Cisco");
    }

    #[test]
    fn parses_linux_ip_neigh() {
        let text = "192.168.1.1 dev eth0 lladdr 00:50:56:c0:00:08 REACHABLE\n192.168.1.30 dev eth0 lladdr b8:27:eb:12:34:56 STALE\nfe80::1 dev eth0 lladdr 00:11:22:33:44:55 router STALE\n";
        let rows = parse_arp_output(text, false);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].vendor, "VMware");
        assert_eq!(rows[1].vendor, "Raspberry Pi");
        assert_eq!(rows[1].state, "stale");
    }

    #[test]
    fn parses_bsd_arp() {
        let text = "? (192.168.1.2) at 0:1f:33:aa:bb:cc on en0 ifscope [ethernet]\n";
        let rows = parse_arp_output(text, false);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].ip, "192.168.1.2");
        assert_eq!(rows[0].vendor, "Netgear");
    }

    #[test]
    fn extracts_ping_latency() {
        assert_eq!(extract_ping_latency("time=12ms ttl=56"), Some(12));
        assert_eq!(extract_ping_latency("time=1.42 ms"), Some(1));
        assert_eq!(extract_ping_latency("time<1ms"), Some(1));
        assert_eq!(extract_ping_latency("unreachable"), None);
    }

    #[test]
    fn local_info_shape() {
        let info = local_network_info().unwrap();
        assert!(cidr::parse_ipv4(&info.local_ip).is_some());
        assert!(info.suggested_cidr.ends_with(".0/24"));
    }

    #[test]
    fn open_port_scan_works_locally() {
        // Bind a listener and confirm the scanner sees it open.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            let _ = listener.accept();
        });
        let r = tcp_scan_port("127.0.0.1", port, 500, false);
        assert_eq!(r.state, PortState::Open);
    }

    #[test]
    fn closed_port_scan_works_locally() {
        // Bind then drop: the port is likely refused (not filtered) on loopback.
        let port = {
            let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            l.local_addr().unwrap().port()
        };
        let r = tcp_scan_port("127.0.0.1", port, 300, false);
        assert_ne!(r.state, PortState::Open);
    }
}
