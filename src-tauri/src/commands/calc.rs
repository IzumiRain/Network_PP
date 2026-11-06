use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TransferResult {
    pub size_bytes: u64,
    pub bandwidth_mbps: f64,
    pub seconds: f64,
    pub human_time: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MtuResult {
    pub mtu: u16,
    pub tcp_mss: u16,
    pub udp_payload_max: u16,
    pub ipv4_min_mtu: u16,
    pub pings_bytes: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverheadResult {
    pub payload_bytes: u64,
    pub frame_bytes: f64,
    pub efficiency_percent: f64,
    pub packets: u64,
    pub total_wire_bytes: u64,
}

fn human_duration(secs: f64) -> String {
    if secs < 1.0 {
        return format!("{:.0} ms", secs * 1000.0);
    }
    if secs < 60.0 {
        return format!("{secs:.1} s");
    }
    let m = (secs / 60.0).floor() as u64;
    let s = secs - m as f64 * 60.0;
    if m < 60 {
        return format!("{m}m {s:.0}s");
    }
    let h = m / 60;
    let m = m % 60;
    if h < 24 {
        return format!("{h}h {m}m");
    }
    format!("{}d {h}h {m}m", h / 24)
}

/// Transfer time: size (bytes) over bandwidth (Mbps — megabits per second,
/// the unit ISP links are sold in).
pub fn transfer_time(size_bytes: u64, bandwidth_mbps: f64) -> Result<TransferResult, String> {
    if size_bytes == 0 {
        return Err("Size must be greater than zero".into());
    }
    if bandwidth_mbps <= 0.0 {
        return Err("Bandwidth must be greater than zero".into());
    }
    let bits = size_bytes as f64 * 8.0;
    let seconds = bits / (bandwidth_mbps * 1_000_000.0);
    Ok(TransferResult {
        size_bytes,
        bandwidth_mbps,
        seconds,
        human_time: human_duration(seconds),
    })
}

/// MSS and related payload limits from an MTU.
pub fn mtu_info(mtu: u16) -> Result<MtuResult, String> {
    if !(576..=65535).contains(&mtu) {
        return Err("MTU must be 576-65535 (IPv4 minimum is 576)".into());
    }
    Ok(MtuResult {
        mtu,
        tcp_mss: mtu - 40, // IPv4 (20) + TCP (20)
        udp_payload_max: mtu - 28, // IPv4 (20) + UDP (8)
        ipv4_min_mtu: 576,
        pings_bytes: format!("{} (TCP-style test: {})", mtu, mtu - 28),
    })
}

/// Per-packet Ethernet+IPv4+TCP efficiency for a payload size.
pub fn overhead(payload_bytes: u64) -> Result<OverheadResult, String> {
    if payload_bytes == 0 {
        return Err("Payload must be greater than zero".into());
    }
    const HEADERS: f64 = 14.0 + 4.0 + 20.0 + 20.0; // Ethernet + FCS + IPv4 + TCP
    const MSS: f64 = 1460.0;
    let packets = (payload_bytes as f64 / MSS).ceil() as u64;
    let mut frame_bytes = 0.0;
    let mut remaining = payload_bytes as f64;
    for _ in 0..packets {
        let seg = remaining.min(MSS);
        frame_bytes += HEADERS + seg;
        remaining -= seg;
    }
    let efficiency = payload_bytes as f64 / frame_bytes * 100.0;
    Ok(OverheadResult {
        payload_bytes,
        frame_bytes,
        efficiency_percent: efficiency,
        packets,
        total_wire_bytes: frame_bytes.round() as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_time_math() {
        // 1 GB over 100 Mbps = 83.85 s
        let r = transfer_time(1_000_000_000, 100.0).unwrap();
        assert!((r.seconds - 80.0).abs() < 5.0, "got {}", r.seconds);
        assert_eq!(r.human_time, "1m 20s");
        let r = transfer_time(1_250_000, 10.0).unwrap();
        assert_eq!(r.human_time, "1.0 s");
        assert!(transfer_time(0, 100.0).is_err());
        assert!(transfer_time(1000, 0.0).is_err());
    }

    #[test]
    fn mtu_values() {
        let r = mtu_info(1500).unwrap();
        assert_eq!(r.tcp_mss, 1460);
        assert_eq!(r.udp_payload_max, 1472);
        assert_eq!(r.ipv4_min_mtu, 576);
        assert_eq!(r.pings_bytes, "1500 (TCP-style test: 1472)");
        assert!(mtu_info(100).is_err());
        assert!(mtu_info(9000).is_ok());
    }

    #[test]
    fn overhead_efficiency() {
        let r = overhead(1460).unwrap();
        assert_eq!(r.packets, 1);
        assert!((r.efficiency_percent - 1460.0 / (58.0 + 1460.0) * 100.0).abs() < 0.01);
        let r = overhead(2920).unwrap();
        assert_eq!(r.packets, 2);
        assert_eq!(r.total_wire_bytes, 3036u64);
        assert!(overhead(0).is_err());
    }

    #[test]
    fn duration_formatting() {
        assert_eq!(human_duration(0.5), "500 ms");
        assert_eq!(human_duration(45.0), "45.0 s");
        assert_eq!(human_duration(125.0), "2m 5s");
        assert_eq!(human_duration(7322.0), "2h 2m");
    }
}
