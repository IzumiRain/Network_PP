use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CidrInfo {
    pub input: String,
    pub ip: String,
    pub prefix: u8,
    pub network: String,
    pub broadcast: String,
    pub netmask: String,
    pub wildcard: String,
    pub first_host: String,
    pub last_host: String,
    pub total_addresses: u64,
    pub usable_hosts: u64,
    pub ip_class: String,
    pub address_type: String,
    pub netmask_binary: String,
    pub ip_binary: String,
    pub network_int: u64,
}

pub fn parse_ipv4(s: &str) -> Option<u32> {
    let mut octets = [0u8; 4];
    let mut count = 0;
    for part in s.trim().split('.') {
        let o: u8 = part.parse().ok()?;
        if count >= 4 || (count == 0 && s.is_empty()) {
            return None;
        }
        octets[count] = o;
        count += 1;
    }
    if count != 4 {
        return None;
    }
    Some(u32::from_be_bytes(octets))
}

pub fn ipv4_string(ip: u32) -> String {
    let b = ip.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}

pub fn mask_of_prefix(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix as u32)
    }
}

pub fn prefix_of_mask(mask: u32) -> Option<u8> {
    let ones = mask.leading_ones() as u8;
    if mask == mask_of_prefix(ones) {
        Some(ones)
    } else {
        None
    }
}

fn address_type(ip: u32) -> &'static str {
    let b = ip.to_be_bytes();
    match b[0] {
        0 => "This network (RFC 791)",
        127 => "Loopback",
        10 => "Private (RFC 1918)",
        169 if b[1] == 254 => "Link-local (APIPA)",
        172 if (16..=31).contains(&b[1]) => "Private (RFC 1918)",
        192 if b[1] == 168 => "Private (RFC 1918)",
        224..=239 => "Multicast",
        240..=255 => "Reserved",
        _ => "Public",
    }
}

fn ip_class(ip: u32) -> &'static str {
    match ip.to_be_bytes()[0] {
        0..=127 => "A",
        128..=191 => "B",
        192..=223 => "C",
        224..=239 => "D (multicast)",
        _ => "E (reserved)",
    }
}

fn binary_u32(v: u32) -> String {
    v.to_be_bytes()
        .iter()
        .map(|b| format!("{b:08b}"))
        .collect::<Vec<_>>()
        .join(".")
}

pub fn calculate(input: &str) -> Result<CidrInfo, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Enter an address like 192.168.1.10/24 or 10.0.0.0 255.255.255.0".into());
    }

    let (ip_str, mask_part) = match input.split_once(['/', ' ', ',']) {
        Some((ip, m)) => (ip, Some(m.trim())),
        None => (input, None),
    };

    let ip = parse_ipv4(ip_str).ok_or_else(|| format!("Invalid IPv4 address: {ip_str}"))?;

    let prefix = match mask_part {
        None => 32,
        Some(m) if m.contains('.') => {
            let mask = parse_ipv4(m).ok_or_else(|| format!("Invalid netmask: {m}"))?;
            prefix_of_mask(mask)
                .ok_or_else(|| format!("Netmask {m} is not contiguous (e.g. 255.255.0.0)"))?
        }
        Some(m) => {
            let p: u8 = m
                .parse()
                .map_err(|_| format!("Invalid prefix: {m} — must be 0-32"))?;
            if p > 32 {
                return Err(format!("/{p} is invalid — prefix must be 0-32"));
            }
            p
        }
    };

    let mask = mask_of_prefix(prefix);
    let network = ip & mask;
    let broadcast = network | !mask;
    let total: u64 = 1u64 << (32 - prefix as u32);
    let (first_host, last_host, usable) = match prefix {
        0..=30 => (
            network + 1,
            broadcast - 1,
            total - 2,
        ),
        31 => (network, broadcast, 2), // RFC 3021 point-to-point
        _ => (network, network, 1),
    };

    Ok(CidrInfo {
        input: input.to_string(),
        ip: ipv4_string(ip),
        prefix,
        network: ipv4_string(network),
        broadcast: ipv4_string(broadcast),
        netmask: ipv4_string(mask),
        wildcard: ipv4_string(!mask),
        first_host: ipv4_string(first_host),
        last_host: ipv4_string(last_host),
        total_addresses: total,
        usable_hosts: usable,
        ip_class: ip_class(ip).to_string(),
        address_type: address_type(ip).to_string(),
        netmask_binary: binary_u32(mask),
        ip_binary: binary_u32(ip),
        network_int: network as u64,
    })
}

pub fn split_subnet(input: &str, new_prefix: u8, limit: usize) -> Result<Vec<String>, String> {
    let info = calculate(input)?;
    if new_prefix < info.prefix || new_prefix > 32 {
        return Err(format!(
            "New prefix must be between {} and 32",
            info.prefix
        ));
    }
    let count: u64 = 1u64 << (new_prefix - info.prefix);
    if count > limit as u64 {
        return Err(format!(
            "Would produce {count} subnets — limit is {limit}. Use a smaller split."
        ));
    }
    let size = 1u32 << (32 - new_prefix as u32);
    let base = parse_ipv4(&info.network).unwrap();
    Ok((0..count)
        .map(|i| format!("{}/{}", ipv4_string(base + (i as u32) * size), new_prefix))
        .collect())
}

pub fn range_to_cidrs(start_str: &str, end_str: &str) -> Result<Vec<String>, String> {
    let start = parse_ipv4(start_str).ok_or_else(|| format!("Invalid start address: {start_str}"))?;
    let end = parse_ipv4(end_str).ok_or_else(|| format!("Invalid end address: {end_str}"))?;
    if end < start {
        return Err("End address is before start address".into());
    }
    let (mut start, end) = (start as u64, end as u64);
    let mut out = Vec::new();
    while start <= end {
        // Largest aligned block that still fits before the end.
        let max_size = 1u64 << start.trailing_zeros().min(32);
        let remaining = end - start + 1;
        let size = max_size.min(remaining.next_power_of_two()).min(1u64 << 32);
        let mut size = size;
        while size > remaining {
            size >>= 1;
        }
        let prefix = 32 - size.trailing_zeros() as u8;
        out.push(format!("{}/{}", ipv4_string(start as u32), prefix));
        start += size;
    }
    Ok(out)
}

pub fn contains(cidr: &str, ip_str: &str) -> Result<bool, String> {
    let info = calculate(cidr)?;
    let ip = parse_ipv4(ip_str).ok_or_else(|| format!("Invalid address: {ip_str}"))?;
    let net = parse_ipv4(&info.network).unwrap();
    let bc = parse_ipv4(&info.broadcast).unwrap();
    Ok(ip >= net && ip <= bc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_addresses() {
        assert_eq!(parse_ipv4("192.168.1.1"), Some(0xC0A80101));
        assert_eq!(parse_ipv4("0.0.0.0"), Some(0));
        assert_eq!(parse_ipv4("255.255.255.255"), Some(u32::MAX));
        assert_eq!(parse_ipv4("256.1.1.1"), None);
        assert_eq!(parse_ipv4("1.2.3"), None);
        assert_eq!(parse_ipv4("1.2.3.4.5"), None);
        assert_eq!(parse_ipv4("a.b.c.d"), None);
    }

    #[test]
    fn roundtrips_ip_strings() {
        for s in ["10.0.0.1", "192.168.100.254", "8.8.8.8"] {
            assert_eq!(ipv4_string(parse_ipv4(s).unwrap()), s);
        }
    }

    #[test]
    fn prefix_mask_roundtrip() {
        for p in 0..=32u8 {
            assert_eq!(prefix_of_mask(mask_of_prefix(p)), Some(p));
        }
        assert_eq!(prefix_of_mask(0x0000FFFF), None); // non-contiguous
    }

    #[test]
    fn calculates_standard_subnet() {
        let c = calculate("192.168.1.10/24").unwrap();
        assert_eq!(c.network, "192.168.1.0");
        assert_eq!(c.broadcast, "192.168.1.255");
        assert_eq!(c.netmask, "255.255.255.0");
        assert_eq!(c.wildcard, "0.0.0.255");
        assert_eq!(c.first_host, "192.168.1.1");
        assert_eq!(c.last_host, "192.168.1.254");
        assert_eq!(c.total_addresses, 256);
        assert_eq!(c.usable_hosts, 254);
        assert_eq!(c.ip_class, "C");
        assert_eq!(c.address_type, "Private (RFC 1918)");
    }

    #[test]
    fn calculates_host_prefixes() {
        let c = calculate("10.0.0.5/31").unwrap();
        assert_eq!(c.usable_hosts, 2); // RFC 3021
        assert_eq!(c.first_host, "10.0.0.4");
        let c = calculate("10.0.0.5/32").unwrap();
        assert_eq!(c.usable_hosts, 1);
        assert_eq!(c.first_host, "10.0.0.5");
    }

    #[test]
    fn accepts_dot_netmask() {
        let c = calculate("172.16.5.4 255.255.240.0").unwrap();
        assert_eq!(c.prefix, 20);
        assert_eq!(c.network, "172.16.0.0");
        assert_eq!(c.broadcast, "172.16.15.255");
    }

    #[test]
    fn rejects_bad_input() {
        assert!(calculate("").is_err());
        assert!(calculate("10.0.0.0/33").is_err());
        assert!(calculate("300.1.1.1/24").is_err());
        assert!(calculate("10.0.0.0 255.255.0.255").is_err()); // non-contiguous
    }

    #[test]
    fn detects_types_and_classes() {
        assert_eq!(calculate("8.8.8.8/32").unwrap().address_type, "Public");
        assert_eq!(calculate("127.0.0.1/8").unwrap().address_type, "Loopback");
        assert_eq!(
            calculate("169.254.1.1/16").unwrap().address_type,
            "Link-local (APIPA)"
        );
        assert_eq!(
            calculate("224.0.0.1/4").unwrap().address_type,
            "Multicast"
        );
        assert_eq!(calculate("100.1.2.3/8").unwrap().ip_class, "A");
    }

    #[test]
    fn splits_subnet() {
        let subs = split_subnet("192.168.0.0/24", 26, 64).unwrap();
        assert_eq!(subs, vec!["192.168.0.0/26", "192.168.0.64/26", "192.168.0.128/26", "192.168.0.192/26"]);
        assert!(split_subnet("192.168.0.0/24", 16, 64).is_err());
        assert!(split_subnet("192.168.0.0/24", 32, 64).is_err()); // 256 > limit
    }

    #[test]
    fn converts_range_to_minimal_cidrs() {
        let out = range_to_cidrs("10.0.0.0", "10.0.0.7").unwrap();
        assert_eq!(out, vec!["10.0.0.0/29"]);
        let out = range_to_cidrs("10.0.0.0", "10.0.0.255").unwrap();
        assert_eq!(out, vec!["10.0.0.0/24"]);
        let out = range_to_cidrs("192.168.1.1", "192.168.1.6").unwrap();
        assert_eq!(
            out,
            vec!["192.168.1.1/32", "192.168.1.2/31", "192.168.1.4/31", "192.168.1.6/32"]
        );
        assert!(range_to_cidrs("10.0.0.5", "10.0.0.1").is_err());
    }

    #[test]
    fn checks_containment() {
        assert!(contains("192.168.1.0/24", "192.168.1.200").unwrap());
        assert!(!contains("192.168.1.0/24", "192.168.2.1").unwrap());
        assert!(contains("0.0.0.0/0", "1.2.3.4").unwrap());
    }
}
