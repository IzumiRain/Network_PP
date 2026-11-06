use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: &'static str,
    pub service: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortReport {
    pub port: u16,
    pub range: String,
    pub assignments: Vec<PortInfo>,
    pub notes: Vec<String>,
}

// Well-known and common registered assignments. TCP/UDP split kept explicit.
const DB: &[(u16, &str, &str, &str)] = &[
    (20, "TCP", "ftp-data", "FTP data channel"),
    (21, "TCP", "ftp", "FTP control"),
    (22, "TCP", "ssh", "SSH remote login / SCP / SFTP"),
    (23, "TCP", "telnet", "Telnet (plaintext, deprecated)"),
    (25, "TCP", "smtp", "SMTP mail transfer"),
    (53, "TCP/UDP", "domain", "DNS"),
    (67, "UDP", "dhcps", "DHCP server"),
    (68, "UDP", "dhcpc", "DHCP client"),
    (69, "UDP", "tftp", "Trivial FTP"),
    (80, "TCP", "http", "HTTP"),
    (88, "TCP", "kerberos", "Kerberos authentication"),
    (110, "TCP", "pop3", "Post Office Protocol v3"),
    (111, "TCP/UDP", "rpcbind", "ONC RPC portmapper"),
    (113, "TCP", "ident", "Ident protocol"),
    (119, "TCP", "nntp", "Network News Transfer"),
    (123, "UDP", "ntp", "Network Time Protocol"),
    (135, "TCP", "msrpc", "Microsoft RPC endpoint mapper"),
    (137, "UDP", "netbios-ns", "NetBIOS name service"),
    (138, "UDP", "netbios-dgm", "NetBIOS datagram"),
    (139, "TCP", "netbios-ssn", "NetBIOS session (SMB over NetBIOS)"),
    (143, "TCP", "imap", "Internet Message Access Protocol"),
    (161, "UDP", "snmp", "SNMP agent"),
    (162, "UDP", "snmptrap", "SNMP trap"),
    (179, "TCP", "bgp", "Border Gateway Protocol"),
    (194, "TCP", "irc", "Internet Relay Chat"),
    (389, "TCP/UDP", "ldap", "Lightweight Directory Access"),
    (427, "TCP/UDP", "slp", "Service Location Protocol"),
    (443, "TCP", "https", "HTTP over TLS"),
    (445, "TCP", "microsoft-ds", "SMB over TCP (Active Directory, file sharing)"),
    (464, "UDP", "kpasswd", "Kerberos password change"),
    (465, "TCP", "smtps", "SMTP over TLS"),
    (500, "UDP", "isakmp", "IKE (IPsec key exchange)"),
    (514, "UDP", "syslog", "Syslog"),
    (515, "TCP", "lpd", "Line Printer Daemon"),
    (548, "TCP", "afp", "Apple Filing Protocol"),
    (554, "TCP", "rtsp", "Real Time Streaming Protocol"),
    (587, "TCP", "submission", "SMTP message submission (STARTTLS)"),
    (631, "TCP/UDP", "ipp", "Internet Printing Protocol (CUPS)"),
    (636, "TCP", "ldaps", "LDAP over TLS"),
    (873, "TCP", "rsync", "rsync daemon"),
    (990, "TCP", "ftps", "FTP over TLS"),
    (993, "TCP", "imaps", "IMAP over TLS"),
    (995, "TCP", "pop3s", "POP3 over TLS"),
    (1080, "TCP", "socks", "SOCKS proxy"),
    (1194, "UDP", "openvpn", "OpenVPN (common default)"),
    (1433, "TCP", "ms-sql-s", "Microsoft SQL Server"),
    (1434, "UDP", "ms-sql-m", "SQL Server browser"),
    (1521, "TCP", "oracle", "Oracle database listener"),
    (1701, "UDP", "l2tp", "Layer 2 Tunneling Protocol"),
    (1723, "TCP", "pptp", "Point-to-Point Tunneling Protocol"),
    (1812, "UDP", "radius", "RADIUS authentication"),
    (1813, "UDP", "radius-acct", "RADIUS accounting"),
    (1900, "UDP", "ssdp", "SSDP (UPnP discovery)"),
    (2049, "TCP/UDP", "nfs", "Network File System"),
    (2082, "TCP", "cpanel", "cPanel (plain)"),
    (2083, "TCP", "cpanels", "cPanel (TLS)"),
    (2181, "TCP", "zookeeper", "Apache ZooKeeper"),
    (2375, "TCP", "docker", "Docker REST API (plain — dangerous if exposed)"),
    (2376, "TCP", "dockers", "Docker REST API over TLS"),
    (2379, "TCP", "etcd-client", "etcd client"),
    (2380, "TCP", "etcd-peer", "etcd peer"),
    (3000, "TCP", "node-dev", "Common Node.js/React dev port (unofficial)"),
    (3128, "TCP", "squid", "Squid HTTP proxy"),
    (3268, "TCP", "gcserver", "Active Directory Global Catalog"),
    (3269, "TCP", "gcserver", "Global Catalog over TLS"),
    (3306, "TCP", "mysql", "MySQL / MariaDB"),
    (3389, "TCP", "ms-wbt-server", "Remote Desktop Protocol (RDP)"),
    (3690, "TCP", "svn", "Subversion"),
    (4444, "TCP", "krb524", "Often used by reverse shells / tools (unofficial)"),
    (4500, "UDP", "ipsec-nat-t", "IPsec NAT traversal"),
    (4713, "TCP", "pulseaudio", "PulseAudio"),
    (4789, "UDP", "vxlan", "VXLAN overlay networking"),
    (5000, "TCP", "upnp/dev", "Common Flask/dev port (unofficial)"),
    (5060, "TCP/UDP", "sip", "Session Initiation Protocol"),
    (5061, "TCP", "sips", "SIP over TLS"),
    (5222, "TCP", "xmpp-client", "XMPP client"),
    (5223, "TCP", "xmpp-client", "XMPP client over TLS"),
    (5353, "UDP", "mdns", "Multicast DNS (Bonjour/Avahi)"),
    (5355, "UDP", "llmnr", "Link-Local Multicast Name Resolution"),
    (5432, "TCP", "postgresql", "PostgreSQL"),
    (5480, "TCP", "appliance", "Common VMware appliance port"),
    (5555, "TCP", "adb", "Android ADB over TCP"),
    (5672, "TCP", "amqp", "AMQP (RabbitMQ)"),
    (5900, "TCP", "vnc", "VNC remote display :0"),
    (5901, "TCP", "vnc", "VNC :1"),
    (5984, "TCP", "couchdb", "Apache CouchDB"),
    (5985, "TCP", "winrm", "Windows Remote Management (HTTP)"),
    (5986, "TCP", "winrm", "Windows Remote Management (HTTPS)"),
    (6379, "TCP", "redis", "Redis"),
    (6443, "TCP", "kubernetes", "Kubernetes API server"),
    (6667, "TCP", "irc", "IRC (common alternative)"),
    (8000, "TCP", "http-alt", "Common dev HTTP port"),
    (8008, "TCP", "http-alt", "Alternative HTTP"),
    (8009, "TCP", "ajp", "Apache JServ Protocol"),
    (8080, "TCP", "http-proxy", "HTTP proxy / alternate web"),
    (8081, "TCP", "http-alt", "Alternate HTTP"),
    (8443, "TCP", "https-alt", "HTTPS alternate"),
    (8888, "TCP", "http-alt", "Jupyter/common alt HTTP"),
    (9000, "TCP", "cslistener", "PHP-FPM / common services"),
    (9042, "TCP", "cassandra", "Apache Cassandra native protocol"),
    (9090, "TCP", "prometheus", "Prometheus / common alt HTTP"),
    (9092, "TCP", "kafka", "Apache Kafka"),
    (9100, "TCP", "jetdirect", "HP printer raw printing"),
    (9200, "TCP", "elasticsearch", "Elasticsearch REST"),
    (9300, "TCP", "elasticsearch", "Elasticsearch transport"),
    (9418, "TCP", "git", "Git daemon"),
    (11211, "TCP/UDP", "memcached", "Memcached"),
    (27017, "TCP", "mongodb", "MongoDB"),
    (32768, "TCP/UDP", "ephemeral-linux", "Linux ephemeral port range start"),
    (49152, "TCP/UDP", "ephemeral-win", "Windows ephemeral port range start"),
];

fn range_of(port: u16) -> &'static str {
    match port {
        0..=1023 => "Well-known (0-1023)",
        1024..=49151 => "Registered (1024-49151)",
        _ => "Dynamic / private (49152-65535)",
    }
}

pub fn lookup(port: u32) -> Result<PortReport, String> {
    if port == 0 || port > 65535 {
        return Err(format!(
            "{port} is not a valid port — ports are 1-65535 (0 is reserved)"
        ));
    }
    let port = port as u16;
    let assignments: Vec<PortInfo> = DB
        .iter()
        .filter(|(p, _, _, _)| *p == port)
        .map(|(p, proto, svc, desc)| PortInfo {
            port: *p,
            protocol: proto,
            service: svc,
            description: desc,
        })
        .collect();
    let mut notes = Vec::new();
    match port {
        0 => notes.push("Reserved — usually means 'any port' or unset in packets".into()),
        1..=1023 => notes.push("Requires admin/root to bind on most systems".into()),
        1024..=49151 => notes.push("Bind freely as unprivileged user".into()),
        _ => notes.push("Ephemeral range — OS picks these for outgoing connections".into()),
    }
    if assignments.is_empty() {
        notes.push("No well-known assignment — check the app registry (IANA) for details".into());
    }
    Ok(PortReport {
        port,
        range: range_of(port).to_string(),
        assignments,
        notes,
    })
}

pub fn search(query: &str) -> Vec<PortInfo> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    DB.iter()
        .filter(|(p, proto, svc, desc)| {
            format!("{p} {proto} {svc} {desc}").to_lowercase().contains(&q)
        })
        .map(|(p, proto, svc, desc)| PortInfo {
            port: *p,
            protocol: proto,
            service: svc,
            description: desc,
        })
        .take(50)
        .collect()
}

pub fn describe_range(from: u32, to: u32) -> Result<String, String> {
    if from == 0 || from > 65535 || to == 0 || to > 65535 {
        return Err("Ports must be 1-65535".into());
    }
    if to < from {
        return Err("End port is before start port".into());
    }
    let count = to - from + 1;
    Ok(format!(
        "{}-{}: {} ports, {}",
        from,
        to,
        count,
        range_of(from as u16)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_common_ports() {
        let r = lookup(22).unwrap();
        assert_eq!(r.assignments.len(), 1);
        assert_eq!(r.assignments[0].service, "ssh");
        assert_eq!(r.range, "Well-known (0-1023)");
        let r = lookup(443).unwrap();
        assert_eq!(r.assignments[0].service, "https");
    }

    #[test]
    fn lookup_rejects_invalid() {
        assert!(lookup(0).is_err());
        assert!(lookup(65536).is_err());
        assert!(lookup(99999).is_err());
        assert!(lookup(8080).is_ok());
    }

    #[test]
    fn unknown_port_gets_range_and_note() {
        let r = lookup(52345).unwrap();
        assert!(r.assignments.is_empty());
        assert_eq!(r.range, "Dynamic / private (49152-65535)");
        assert!(r.notes.iter().any(|n| n.contains("Ephemeral")));
        let r = lookup(45678).unwrap();
        assert_eq!(r.range, "Registered (1024-49151)");
    }

    #[test]
    fn search_finds_services() {
        let hits = search("dns");
        assert!(hits.iter().any(|h| h.port == 53));
        let hits = search("vnc");
        assert!(hits.iter().any(|h| h.port == 5900));
        assert!(search("").is_empty());
    }

    #[test]
    fn describe_range_validates() {
        assert_eq!(
            describe_range(80, 80).unwrap(),
            "80-80: 1 ports, Well-known (0-1023)"
        );
        assert!(describe_range(0, 80).is_err());
        assert!(describe_range(80, 10).is_err());
    }
}
