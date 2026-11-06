// Minimal PCAP file reader (classic format, RFC/levine's spec):
// 24-byte global header + per-packet 16-byte headers. Handles both byte
// orders and micro/nano resolution. Packet bytes feed into packets::parse.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PcapPacket {
    pub index: u32,
    pub ts_sec: u64,
    pub ts_frac_display: String,
    pub incl_len: u32,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PcapInfo {
    pub file_version: String,
    pub linktype: String,
    pub snaplen: u32,
    pub packet_count: usize,
    pub packets: Vec<PcapPacket>,
    pub duration_secs: Option<f64>,
    pub truncated: bool,
}

const MAX_PACKETS: usize = 2000;

fn linktype_name(v: u32) -> String {
    match v {
        1 => "Ethernet".into(),
        101 => "Raw IP".into(),
        12 | 14 => "Raw (RadioTap/loop)".into(),
        113 => "Linux SLL".into(),
        228 => "IPv4".into(),
        229 => "IPv6".into(),
        other => format!("DLT {other}"),
    }
}

pub fn parse_pcap(bytes: &[u8]) -> Result<PcapInfo, String> {
    if bytes.len() < 24 {
        return Err("file too small — not a pcap (needs a 24-byte header)".into());
    }
    let magic = &bytes[0..4];
    let (swap, nanos) = match magic {
        [0xa1, 0xb2, 0xc3, 0xd4] => (false, false), // big-endian usec
        [0xd4, 0xc3, 0xb2, 0xa1] => (true, false),  // little-endian usec
        [0xa1, 0xb2, 0x3c, 0x4d] => (false, true),  // big-endian nanos
        [0x4d, 0x3c, 0xb2, 0xa1] => (true, true),   // little-endian nanos
        [0x0a, 0x0d, 0x0d, 0x0a] => {
            return Err("this is a pcapng file — classic pcap is supported".into())
        }
        other => return Err(format!("unknown pcap magic {:02x?}", other)),
    };
    let rd16 = |b: &[u8]| -> u16 {
        let v = u16::from_be_bytes([b[0], b[1]]);
        if swap { v.swap_bytes() } else { v }
    };
    let rd32 = |b: &[u8]| -> u32 {
        let v = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        if swap { v.swap_bytes() } else { v }
    };

    let vmaj = rd16(&bytes[4..6]);
    let vmin = rd16(&bytes[6..8]);
    let _tz = rd32(&bytes[8..12]);
    let _sig = rd32(&bytes[12..16]);
    let snaplen = rd32(&bytes[16..20]);
    let linktype = rd32(&bytes[20..24]);

    let mut packets = Vec::new();
    let mut pos = 24usize;
    let mut index = 0u32;
    let mut first_ts: Option<f64> = None;
    let mut last_ts: Option<f64> = None;
    let mut truncated = false;

    while pos + 16 <= bytes.len() {
        let ts_sec = rd32(&bytes[pos..pos + 4]) as u64;
        let ts_frac = rd32(&bytes[pos + 4..pos + 8]);
        let incl_len = rd32(&bytes[pos + 8..pos + 12]) as usize;
        let _orig_len = rd32(&bytes[pos + 12..pos + 16]) as usize;
        pos += 16;

        if incl_len > bytes.len().saturating_sub(pos) {
            truncated = true;
            break;
        }
        if incl_len > 262_144 {
            truncated = true;
            break; // implausible packet — header desync
        }
        let data = &bytes[pos..pos + incl_len];
        pos += incl_len;

        let frac_display = if nanos {
            format!("{ts_sec}.{ts_frac:09}")
        } else {
            format!("{ts_sec}.{ts_frac:06}")
        };
        let ts = ts_sec as f64
            + ts_frac as f64 / if nanos { 1e9 } else { 1e6 };
        first_ts.get_or_insert(ts);
        last_ts = Some(ts);

        let summary = if linktype == 1 && data.len() >= 14 {
            crate::commands::packets::parse(data)
                .map(|p| packet_summary(&p))
                .unwrap_or_else(|e| format!("unparsed ({e})"))
        } else {
            format!("{} bytes", data.len())
        };

        packets.push(PcapPacket {
            index,
            ts_sec,
            ts_frac_display: frac_display,
            incl_len: incl_len as u32,
            summary,
        });
        index += 1;
        if packets.len() >= MAX_PACKETS {
            truncated = true;
            break;
        }
    }

    let duration_secs = match (first_ts, last_ts) {
        (Some(a), Some(b)) => Some((b - a).max(0.0)),
        _ => None,
    };

    Ok(PcapInfo {
        file_version: format!("{vmaj}.{vmin}"),
        linktype: linktype_name(linktype),
        snaplen,
        packet_count: packets.len(),
        packets,
        duration_secs,
        truncated,
    })
}

fn packet_summary(p: &crate::commands::packets::Packet) -> String {
    use crate::commands::packets::{NetworkLayer, TransportLayer};
    let net = match &p.network {
        Some(NetworkLayer::Ipv4(h)) => format!("{} → {}", h.source, h.destination),
        Some(NetworkLayer::Ipv6(h)) => format!("{} → {}", h.source, h.destination),
        None => return "non-IP frame".into(),
    };
    let trans = match &p.transport {
        Some(TransportLayer::Tcp(t)) => format!(
            "TCP {}→{} {}",
            t.source_port,
            t.destination_port,
            t.flag_names.join("+")
        ),
        Some(TransportLayer::Udp(u)) => format!("UDP {}→{}", u.source_port, u.destination_port),
        Some(TransportLayer::Icmp(i)) => format!("ICMP {}", i.type_name),
        None => String::new(),
    };
    format!("{net} · {trans}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_pcap(swap: bool, nanos: bool) -> Vec<u8> {
        let wr32 = |v: u32| -> [u8; 4] {
            let mut b = v.to_be_bytes();
            if swap { b.reverse(); }
            b
        };
        let wr16 = |v: u16| -> [u8; 2] {
            let mut b = v.to_be_bytes();
            if swap { b.reverse(); }
            b
        };
        let mut b = Vec::new();
        // magic
        match (swap, nanos) {
            (true, false) => b.extend_from_slice(&[0xd4, 0xc3, 0xb2, 0xa1]),
            (false, false) => b.extend_from_slice(&[0xa1, 0xb2, 0xc3, 0xd4]),
            (true, true) => b.extend_from_slice(&[0x4d, 0x3c, 0xb2, 0xa1]),
            (false, true) => b.extend_from_slice(&[0xa1, 0xb2, 0x3c, 0x4d]),
        }
        b.extend_from_slice(&wr16(2)); // vmaj
        b.extend_from_slice(&wr16(4)); // vmin
        b.extend_from_slice(&wr32(0)); // tz
        b.extend_from_slice(&wr32(0)); // sigfigs
        b.extend_from_slice(&wr32(65535)); // snaplen
        b.extend_from_slice(&wr32(1)); // ethernet
        // one packet: an Ethernet+IPv4+TCP SYN (reuse the packets test builder)
        let pkt = crate::commands::packets::build_ipv4_tcp_packet(
            "10.0.0.1",
            "10.0.0.2",
            51000,
            443,
            crate::commands::packets::TcpFlags { syn: true, ..Default::default() },
            &[],
        )
        .unwrap();
        b.extend_from_slice(&wr32(1_700_000_000)); // ts_sec
        b.extend_from_slice(&wr32(if nanos { 500_000_000 } else { 500_000 })); // frac
        b.extend_from_slice(&wr32(pkt.len() as u32));
        b.extend_from_slice(&wr32(pkt.len() as u32));
        b.extend_from_slice(&pkt);
        b
    }

    #[test]
    fn parses_all_byte_orders() {
        for (swap, nanos) in [(false, false), (true, false), (false, true), (true, true)] {
            let info = parse_pcap(&build_pcap(swap, nanos)).unwrap();
            assert_eq!(info.packet_count, 1, "swap={swap} nanos={nanos}");
            assert_eq!(info.linktype, "Ethernet");
            assert_eq!(info.file_version, "2.4");
            // single packet: duration is zero; the 0.5 lives in the fraction
            assert_eq!(info.duration_secs.unwrap(), 0.0);
            let frac = &info.packets[0].ts_frac_display;
            assert!(frac.ends_with(".500000") || frac.ends_with(".500000000"), "frac={frac}");
            assert!(info.packets[0].summary.contains("TCP 51000→443 SYN"));
            assert!(info.packets[0].summary.contains("10.0.0.1 → 10.0.0.2"));
        }
    }

    #[test]
    fn rejects_pcapng_and_short_files() {
        let mut ng = Vec::new();
        ng.extend_from_slice(&[0x0a, 0x0d, 0x0d, 0x0a]);
        ng.extend_from_slice(&[0u8; 40]);
        assert!(parse_pcap(&ng).unwrap_err().contains("pcapng"));
        assert!(parse_pcap(&[0u8; 10]).unwrap_err().contains("too small"));
        assert!(parse_pcap(&[0xff; 24]).is_err());
    }

    #[test]
    fn truncated_packet_detected() {
        let mut b = build_pcap(false, false);
        b.truncate(24 + 16 + 10); // header says more bytes than remain
        let info = parse_pcap(&b).unwrap();
        assert!(info.truncated);
        assert_eq!(info.packet_count, 0);
    }
}
