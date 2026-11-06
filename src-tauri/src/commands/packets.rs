use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MacAddr(pub [u8; 6]);

impl std::fmt::Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EthernetHeader {
    pub destination: String,
    pub source: String,
    pub ethertype: u16,
    pub ethertype_name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Ipv4Header {
    pub version: u8,
    pub ihl: u8, // header length in 32-bit words
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub dont_fragment: bool,
    pub more_fragments: bool,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub protocol_name: String,
    pub header_checksum: u16,
    pub source: String,
    pub destination: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Ipv6Header {
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub next_header_name: String,
    pub hop_limit: u8,
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
pub struct TcpFlags {
    pub fin: bool,
    pub syn: bool,
    pub rst: bool,
    pub psh: bool,
    pub ack: bool,
    pub urg: bool,
    pub ece: bool,
    pub cwr: bool,
}

impl TcpFlags {
    pub fn to_byte(self) -> u8 {
        let mut b = 0u8;
        for (i, f) in [
            self.fin, self.syn, self.rst, self.psh, self.ack, self.urg, self.ece, self.cwr,
        ]
        .iter()
        .enumerate()
        {
            if *f {
                b |= 1 << i;
            }
        }
        b
    }
    pub fn names(self) -> Vec<&'static str> {
        let all = [
            ("FIN", self.fin),
            ("SYN", self.syn),
            ("RST", self.rst),
            ("PSH", self.psh),
            ("ACK", self.ack),
            ("URG", self.urg),
            ("ECE", self.ece),
            ("CWR", self.cwr),
        ];
        all.into_iter().filter(|(_, v)| *v).map(|(n, _)| n).collect()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TcpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence: u32,
    pub acknowledgment: u32,
    pub data_offset: u8, // in 32-bit words
    pub flags: TcpFlags,
    pub flag_names: Vec<&'static str>,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct IcmpHeader {
    pub message_type: u8,
    pub code: u8,
    pub type_name: String,
    pub rest_of_header: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: u16,
    pub reason: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", content = "data")]
pub enum AppLayer {
    #[serde(rename = "http-request")]
    HttpRequest(HttpRequest),
    #[serde(rename = "http-response")]
    HttpResponse(HttpResponse),
    #[serde(rename = "raw")]
    Raw,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", content = "data")]
pub enum TransportLayer {
    #[serde(rename = "tcp")]
    Tcp(TcpHeader),
    #[serde(rename = "udp")]
    Udp(UdpHeader),
    #[serde(rename = "icmp")]
    Icmp(IcmpHeader),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", content = "data")]
pub enum NetworkLayer {
    #[serde(rename = "ipv4")]
    Ipv4(Ipv4Header),
    #[serde(rename = "ipv6")]
    Ipv6(Ipv6Header),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Packet {
    pub link: Option<EthernetHeader>,
    pub network: Option<NetworkLayer>,
    pub transport: Option<TransportLayer>,
    pub application: Option<AppLayer>,
    pub payload_hex: String,
    pub payload_ascii: String,
    pub total_length: usize,
}

pub fn protocol_name(proto: u8) -> String {
    match proto {
        1 => "ICMP".into(),
        2 => "IGMP".into(),
        6 => "TCP".into(),
        9 => "IGP".into(),
        17 => "UDP".into(),
        41 => "IPv6".into(),
        47 => "GRE".into(),
        50 => "ESP".into(),
        51 => "AH".into(),
        58 => "IPv6-ICMP".into(),
        89 => "OSPF".into(),
        132 => "SCTP".into(),
        _ => format!("Proto {proto}"),
    }
}

fn ethertype_name(v: u16) -> String {
    match v {
        0x0800 => "IPv4".into(),
        0x0806 => "ARP".into(),
        0x86DD => "IPv6".into(),
        0x8100 => "VLAN (802.1Q)".into(),
        _ => format!("0x{v:04X}"),
    }
}

pub fn ones_complement(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    for pair in data.chunks(2) {
        let w = if pair.len() == 2 {
            u16::from_be_bytes([pair[0], pair[1]]) as u32
        } else {
            (pair[0] as u32) << 8
        };
        sum += w;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

fn ipv4_checksum(header: &[u8]) -> u16 {
    ones_complement(header)
}

fn transport_checksum(src: u32, dst: u32, proto: u8, segment: &[u8]) -> u16 {
    let mut pseudo = Vec::with_capacity(12 + segment.len());
    pseudo.extend_from_slice(&src.to_be_bytes());
    pseudo.extend_from_slice(&dst.to_be_bytes());
    pseudo.push(0);
    pseudo.push(proto);
    pseudo.extend_from_slice(&(segment.len() as u16).to_be_bytes());
    pseudo.extend_from_slice(segment);
    let c = ones_complement(&pseudo);
    if c == 0 && proto == 17 {
        0xFFFF // UDP: a computed zero is transmitted as all-ones
    } else {
        c
    }
}

pub fn hex_dump(data: &[u8]) -> String {
    let mut out = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        let mut hex_str = hex.join(" ");
        if chunk.len() < 16 {
            hex_str.push_str(&" ".repeat((16 - chunk.len()) * 3));
        }
        let ascii: String = chunk
            .iter()
            .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
            .collect();
        out.push_str(&format!("{:08x}  {}  {}\n", i * 16, hex_str, ascii));
    }
    out
}

fn ascii_of(data: &[u8]) -> String {
    data.iter()
        .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
        .collect()
}

fn parse_http(payload: &[u8]) -> Option<AppLayer> {
    let text = std::str::from_utf8(payload).ok()?;
    let is_request = text.starts_with("GET ")
        || text.starts_with("POST ")
        || text.starts_with("PUT ")
        || text.starts_with("DELETE ")
        || text.starts_with("HEAD ")
        || text.starts_with("OPTIONS ")
        || text.starts_with("PATCH ")
        || text.starts_with("TRACE ");
    let is_response = text.starts_with("HTTP/");
    if !is_request && !is_response {
        return None;
    }
    let (head, body) = match text.find("\r\n\r\n") {
        Some(i) => (&text[..i], &text[i + 4..]),
        None => (text, ""),
    };
    let mut lines = head.lines();
    if is_request {
        let request_line = lines.next()?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next()?.to_string();
        let target = parts.next()?.to_string();
        let version = parts.next().unwrap_or("HTTP/1.1").to_string();
        let headers = lines
            .filter_map(|l| l.split_once(": "))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Some(AppLayer::HttpRequest(HttpRequest {
            method,
            target,
            version,
            headers,
            body: body.to_string(),
        }))
    } else {
        let status_line = lines.next()?;
        let mut parts = status_line.splitn(3, ' ');
        let version = parts.next()?.to_string();
        let status_code = parts.next()?.parse().ok()?;
        let reason = parts.next().unwrap_or("").to_string();
        let headers = lines
            .filter_map(|l| l.split_once(": "))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Some(AppLayer::HttpResponse(HttpResponse {
            version,
            status_code,
            reason,
            headers,
            body: body.to_string(),
        }))
    }
}

pub fn parse(data: &[u8]) -> Result<Packet, String> {
    parse_l3(data)
}

fn parse_l3(data: &[u8]) -> Result<Packet, String> {
    // Try Ethernet first (need 14 bytes), else treat input as a raw IP datagram.
    let (net_bytes, link) = if data.len() >= 14 {
        let et = u16::from_be_bytes([data[12], data[13]]);
        if et == 0x0800 || et == 0x86DD {
            (
                &data[14..],
                Some(EthernetHeader {
                    destination: MacAddr([data[0], data[1], data[2], data[3], data[4], data[5]])
                        .to_string(),
                    source: MacAddr([data[6], data[7], data[8], data[9], data[10], data[11]])
                        .to_string(),
                    ethertype: et,
                    ethertype_name: ethertype_name(et),
                }),
            )
        } else {
            (data, None)
        }
    } else {
        (data, None)
    };

    if net_bytes.is_empty() {
        return Err("No network layer data after link header".into());
    }

    let version = net_bytes[0] >> 4;
    let (network, transport_bytes) = match version {
        4 => parse_ipv4_layer(net_bytes)?,
        6 => parse_ipv6_layer(net_bytes)?,
        _ => return Err(format!("Unknown IP version {version} — expected 4 or 6")),
    };

    let payload_after_transport = parse_transport(&network, transport_bytes)?;
    let transport = payload_after_transport
        .and_then(|_| transport_of(&network, transport_bytes));
    let (app, payload) = match payload_after_transport {
        Some(t) if t.is_empty() => (None, t),
        Some(t) => match parse_http(t) {
            Some(app) => (Some(app), t),
            None => (Some(AppLayer::Raw), t),
        },
        None => (None, &[][..]),
    };

    Ok(Packet {
        link,
        network: Some(network),
        transport,
        application: app,
        payload_hex: hex_to_string(payload),
        payload_ascii: ascii_of(payload),
        total_length: data.len(),
    })
}

fn hex_to_string(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect()
}

fn parse_ipv4_layer(data: &[u8]) -> Result<(NetworkLayer, &[u8]), String> {
    if data.len() < 20 {
        return Err("IPv4 header truncated — need at least 20 bytes".into());
    }
    let ihl_words = (data[0] & 0x0F) as usize;
    if ihl_words < 5 {
        return Err(format!("Invalid IHL {ihl_words} — must be at least 5"));
    }
    let hlen = ihl_words * 4;
    if data.len() < hlen {
        return Err("IPv4 header length exceeds data".into());
    }
    let total_length = u16::from_be_bytes([data[2], data[3]]);
    let flags_frag = u16::from_be_bytes([data[6], data[7]]);
    let proto = data[9];
    let src = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
    let dst = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let options = data[20..hlen]
        .chunks(4)
        .map(|c| c.iter().map(|b| format!("{b:02x}")).collect::<String>())
        .collect();
    let header = Ipv4Header {
        version: 4,
        ihl: ihl_words as u8,
        dscp: data[1] >> 2,
        ecn: data[1] & 0b11,
        total_length,
        identification: u16::from_be_bytes([data[4], data[5]]),
        dont_fragment: flags_frag & 0x4000 != 0,
        more_fragments: flags_frag & 0x2000 != 0,
        fragment_offset: flags_frag & 0x1FFF,
        ttl: data[8],
        protocol: proto,
        protocol_name: protocol_name(proto),
        header_checksum: u16::from_be_bytes([data[10], data[11]]),
        source: crate::commands::cidr::ipv4_string(src),
        destination: crate::commands::cidr::ipv4_string(dst),
        options,
    };
    let end = (total_length as usize).min(data.len()).max(hlen);
    Ok((NetworkLayer::Ipv4(header), &data[hlen..end]))
}

fn parse_ipv6_layer(data: &[u8]) -> Result<(NetworkLayer, &[u8]), String> {
    if data.len() < 40 {
        return Err("IPv6 header truncated — need 40 bytes".into());
    }
    let traffic_flow = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let next = data[6];
    let payload_len = u16::from_be_bytes([data[4], data[5]]) as usize;
    let src = format_ipv6(&data[8..24]);
    let dst = format_ipv6(&data[24..40]);
    let header = Ipv6Header {
        traffic_class: (traffic_flow >> 20) as u8,
        flow_label: traffic_flow & 0x000F_FFFF,
        payload_length: payload_len as u16,
        next_header: next,
        next_header_name: protocol_name(next),
        hop_limit: data[7],
        source: src,
        destination: dst,
    };
    let end = (40 + payload_len).min(data.len());
    Ok((NetworkLayer::Ipv6(header), &data[40..end]))
}

fn format_ipv6(b: &[u8]) -> String {
    let groups: Vec<String> = b.chunks(2).map(|c| format!("{:x}", u16::from_be_bytes([c[0], c[1]]))).collect();
    groups.join(":")
}

fn parse_transport<'a>(network: &NetworkLayer, data: &'a [u8]) -> Result<Option<&'a [u8]>, String> {
    let proto = match network {
        NetworkLayer::Ipv4(h) => h.protocol,
        NetworkLayer::Ipv6(h) => h.next_header,
    };
    match proto {
        6 => {
            if data.len() < 20 {
                return Err("TCP header truncated — need at least 20 bytes".into());
            }
            let offset = ((data[12] >> 4) as usize) * 4;
            if data.len() < offset {
                return Err("TCP header length exceeds data".into());
            }
            Ok(Some(&data[offset..]))
        }
        17 => {
            if data.len() < 8 {
                return Err("UDP header truncated — need 8 bytes".into());
            }
            let len = u16::from_be_bytes([data[4], data[5]]) as usize;
            if len < 8 {
                return Err(format!("UDP length {len} invalid — must be >= 8"));
            }
            Ok(Some(&data[8..len.min(data.len())]))
        }
        1 | 58 => {
            if data.len() < 8 {
                return Err("ICMP header truncated — need 8 bytes".into());
            }
            Ok(Some(&data[8..]))
        }
        _ => Ok(Some(data)),
    }
}

fn transport_of(network: &NetworkLayer, data: &[u8]) -> Option<TransportLayer> {
    let proto = match network {
        NetworkLayer::Ipv4(h) => h.protocol,
        NetworkLayer::Ipv6(h) => h.next_header,
    };
    match proto {
        6 => {
            let flags = TcpFlags {
                fin: data[13] & 0x01 != 0,
                syn: data[13] & 0x02 != 0,
                rst: data[13] & 0x04 != 0,
                psh: data[13] & 0x08 != 0,
                ack: data[13] & 0x10 != 0,
                urg: data[13] & 0x20 != 0,
                ece: data[13] & 0x40 != 0,
                cwr: data[13] & 0x80 != 0,
            };
            let flag_names = flags.names();
            Some(TransportLayer::Tcp(TcpHeader {
                source_port: u16::from_be_bytes([data[0], data[1]]),
                destination_port: u16::from_be_bytes([data[2], data[3]]),
                sequence: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
                acknowledgment: u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
                data_offset: data[12] >> 4,
                flags,
                flag_names,
                window_size: u16::from_be_bytes([data[14], data[15]]),
                checksum: u16::from_be_bytes([data[16], data[17]]),
                urgent_pointer: u16::from_be_bytes([data[18], data[19]]),
                options: data[20..((data[12] >> 4) as usize * 4).min(data.len())]
                    .chunks(4)
                    .filter(|c| c.iter().any(|&b| b != 0))
                    .map(|c| c.iter().map(|b| format!("{b:02x}")).collect::<String>())
                    .collect(),
            }))
        }
        17 => Some(TransportLayer::Udp(UdpHeader {
            source_port: u16::from_be_bytes([data[0], data[1]]),
            destination_port: u16::from_be_bytes([data[2], data[3]]),
            length: u16::from_be_bytes([data[4], data[5]]),
            checksum: u16::from_be_bytes([data[6], data[7]]),
        })),
        1 | 58 => Some(TransportLayer::Icmp(IcmpHeader {
            message_type: data[0],
            code: data[1],
            type_name: icmp_name(data[0]),
            rest_of_header: data[4..8]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>(),
        })),
        _ => None,
    }
}

fn icmp_name(t: u8) -> String {
    match t {
        0 => "Echo Reply".into(),
        3 => "Destination Unreachable".into(),
        5 => "Redirect".into(),
        8 => "Echo Request".into(),
        9 => "Router Advertisement".into(),
        10 => "Router Solicitation".into(),
        11 => "Time Exceeded".into(),
        128 => "Echo Request (v6)".into(),
        129 => "Echo Reply (v6)".into(),
        _ => format!("Type {t}"),
    }
}

// ---- build ----

pub fn build_ipv4_tcp_packet(src: &str, dst: &str, sport: u16, dport: u16, flags: TcpFlags, payload: &[u8]) -> Result<Vec<u8>, String> {
    let src_ip = crate::commands::cidr::parse_ipv4(src).ok_or_else(|| format!("Invalid source IP: {src}"))?;
    let dst_ip = crate::commands::cidr::parse_ipv4(dst).ok_or_else(|| format!("Invalid destination IP: {dst}"))?;
    let tcp_len = 20 + payload.len();
    let total = 20 + tcp_len as u16;

    // TCP segment with pseudo-header checksum
    let mut tcp = Vec::with_capacity(tcp_len);
    tcp.extend_from_slice(&sport.to_be_bytes());
    tcp.extend_from_slice(&dport.to_be_bytes());
    tcp.extend_from_slice(&0x11223344u32.to_be_bytes()); // placeholder seq
    tcp.extend_from_slice(&0u32.to_be_bytes()); // ack
    tcp.push(5 << 4); // data offset 5 words
    tcp.push(flags.to_byte());
    tcp.extend_from_slice(&65535u16.to_be_bytes()); // window
    tcp.extend_from_slice(&0u16.to_be_bytes()); // checksum placeholder
    tcp.extend_from_slice(&0u16.to_be_bytes()); // urgent
    tcp.extend_from_slice(payload);
    let checksum = transport_checksum(src_ip, dst_ip, 6, &tcp);
    tcp[16..18].copy_from_slice(&checksum.to_be_bytes());

    let mut ip = Vec::with_capacity(20 + tcp_len);
    ip.push(0x45); // v4, IHL 5
    ip.push(0); // DSCP/ECN
    ip.extend_from_slice(&total.to_be_bytes());
    ip.extend_from_slice(&0x1C46u16.to_be_bytes()); // id placeholder
    ip.extend_from_slice(&0x4000u16.to_be_bytes()); // DF set
    ip.push(64); // TTL
    ip.push(6); // TCP
    ip.extend_from_slice(&0u16.to_be_bytes()); // checksum placeholder
    ip.extend_from_slice(&src_ip.to_be_bytes());
    ip.extend_from_slice(&dst_ip.to_be_bytes());
    let c = ipv4_checksum(&ip);
    ip[10..12].copy_from_slice(&c.to_be_bytes());
    ip.extend_from_slice(&tcp);
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eth_ip_tcp_syn() -> Vec<u8> {
        // Ethernet (dst 6, src 6, ethertype 0x0800) + IPv4 + TCP SYN
        let mut v = Vec::new();
        v.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        v.extend_from_slice(&[0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e]);
        v.extend_from_slice(&[0x08, 0x00]);
        let inner = build_ipv4_tcp_packet(
            "192.168.1.50",
            "93.184.216.34",
            51000,
            443,
            TcpFlags { syn: true, ..Default::default() },
            &[],
        )
        .unwrap();
        v.extend_from_slice(&inner);
        v
    }

    #[test]
    fn ones_complement_known_value() {
        // Classic example: sum of 0x0001, 0xf203, 0xf4f5, 0xf6f7 -> checksum 0x220d
        let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
        assert_eq!(ones_complement(&data), 0x220d);
    }

    #[test]
    fn builds_and_parses_ethernet_ipv4_tcp() {
        let bytes = eth_ip_tcp_syn();
        let p = parse(&bytes).unwrap();
        assert!(p.link.is_some());
        let link = p.link.unwrap();
        assert_eq!(link.destination, "FF:FF:FF:FF:FF:FF");
        assert_eq!(link.source, "00:1A:2B:3C:4D:5E");
        assert_eq!(link.ethertype_name, "IPv4");

        match p.network.unwrap() {
            NetworkLayer::Ipv4(h) => {
                assert_eq!(h.source, "192.168.1.50");
                assert_eq!(h.destination, "93.184.216.34");
                assert_eq!(h.ttl, 64);
                assert_eq!(h.protocol_name, "TCP");
                assert_eq!(h.dont_fragment, true);
            }
            _ => panic!("expected ipv4"),
        }

        match p.transport.unwrap() {
            TransportLayer::Tcp(t) => {
                assert_eq!(t.source_port, 51000);
                assert_eq!(t.destination_port, 443);
                assert!(t.flags.syn && !t.flags.ack);
                assert_eq!(t.flags.names(), vec!["SYN"]);
            }
            _ => panic!("expected tcp"),
        }
    }

    #[test]
    fn tcp_checksum_verifies_against_reparse() {
        // Build, zero the checksum, recompute via pseudo-header path and compare.
        let ip = build_ipv4_tcp_packet(
            "10.0.0.1",
            "10.0.0.2",
            1234,
            80,
            TcpFlags { syn: true, ack: true, ..Default::default() },
            b"hello",
        )
        .unwrap();
        let tcp = &ip[20..];
        let stored = u16::from_be_bytes([tcp[16], tcp[17]]);
        let src = u32::from_be_bytes([ip[12], ip[13], ip[14], ip[15]]);
        let dst = u32::from_be_bytes([ip[16], ip[17], ip[18], ip[19]]);
        let mut seg = tcp.to_vec();
        seg[16] = 0;
        seg[17] = 0;
        let computed = transport_checksum(src, dst, 6, &seg);
        assert_eq!(stored, computed);
        // Verify by parsing back the payload too.
        let p = parse(&ip).unwrap();
        assert_eq!(p.payload_ascii, "hello");
    }

    #[test]
    fn parses_http_request_over_tcp() {
        let body = b"GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: netpp\r\n\r\n";
        let ip = build_ipv4_tcp_packet("10.0.0.9", "10.0.0.8", 49152, 80, TcpFlags { psh: true, ack: true, ..Default::default() }, body).unwrap();
        let p = parse(&ip).unwrap();
        match p.application.clone().unwrap() {
            AppLayer::HttpRequest(r) => {
                assert_eq!(r.method, "GET");
                assert_eq!(r.target, "/index.html");
                assert_eq!(r.version, "HTTP/1.1");
                assert!(r.headers.contains(&("Host".into(), "example.com".into())));
            }
            _ => panic!("expected http request, got {:?}", p.application),
        }
    }

    #[test]
    fn parses_ipv6_header() {
        let mut v = vec![0x60, 0x00, 0x00, 0x00]; // v6, TC 0, flow 0
        v.extend_from_slice(&8u16.to_be_bytes()); // payload length
        v.push(17); // UDP
        v.push(64); // hop limit
        v.extend_from_slice(&[0x20; 16]); // src
        v.extend_from_slice(&[0x40; 16]); // dst
        v.extend_from_slice(&68u16.to_be_bytes()); // sport
        v.extend_from_slice(&53u16.to_be_bytes()); // dport
        v.extend_from_slice(&16u16.to_be_bytes()); // udp length
        v.extend_from_slice(&0u16.to_be_bytes()); // checksum
        v.extend_from_slice(&[0x00, 0x01, 0x02, 0x03]); // udp payload
        let p = parse(&v).unwrap();
        assert!(p.link.is_none());
        match p.network.unwrap() {
            NetworkLayer::Ipv6(h) => {
                assert_eq!(h.next_header_name, "UDP");
                assert_eq!(h.hop_limit, 64);
                assert_eq!(h.source, "2020:2020:2020:2020:2020:2020:2020:2020");
            }
            _ => panic!("expected ipv6"),
        }
        match p.transport.unwrap() {
            TransportLayer::Udp(u) => {
                assert_eq!(u.destination_port, 53);
                assert_eq!(u.length, 16);
            }
            _ => panic!("expected udp"),
        }
    }

    #[test]
    fn rejects_truncated_headers() {
        assert!(parse(&[0x45]).is_err());
        assert!(parse(&vec![0u8; 10]).is_err()); // too short for eth or ip
        let mut v = eth_ip_tcp_syn();
        v.truncate(30); // cuts into TCP header
        assert!(parse(&v).is_err());
    }

    #[test]
    fn hex_dump_format() {
        let s = hex_dump(b"AB");
        assert!(s.starts_with("00000000  41 42"));
        assert!(s.contains("AB"));
        let s = hex_dump(&[]);
        assert_eq!(s, "");
    }
}
