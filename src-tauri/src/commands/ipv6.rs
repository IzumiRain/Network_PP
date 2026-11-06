use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ipv6Info {
    pub input: String,
    pub compressed: String,
    pub expanded: String,
    pub prefix: u8,
    pub network: String,
    pub first_address: String,
    pub last_address: String,
    pub total_addresses: String,
    pub address_type: String,
    pub binary: String,
}

fn parse_with_prefix(input: &str) -> Result<(u128, u8), String> {
    let input = input.trim();
    let (addr_str, prefix) = match input.rsplit_once('/') {
        Some((a, p)) => {
            let p: u8 = p
                .trim()
                .parse()
                .map_err(|_| format!("Invalid prefix '{p}' — must be 0-128"))?;
            if p > 128 {
                return Err("/129 is invalid — prefix must be 0-128".into());
            }
            (a, p)
        }
        None => (input, 128),
    };
    let addr: std::net::Ipv6Addr = addr_str
        .trim()
        .parse()
        .map_err(|_| format!("Invalid IPv6 address: {addr_str}"))?;
    Ok((u128::from(addr), prefix))
}

fn expand(addr: u128) -> String {
    let b = addr.to_be_bytes();
    let groups: Vec<String> = b.chunks(2).map(|c| format!("{:04x}", u16::from_be_bytes([c[0], c[1]]))).collect();
    groups.join(":")
}

/// RFC 5952 compression: longest zero run replaced by ::, lowercase.
pub fn compress(addr: u128) -> String {
    let groups: [u16; 8] = {
        let b = addr.to_be_bytes();
        let mut g = [0u16; 8];
        for (i, c) in b.chunks(2).enumerate() {
            g[i] = u16::from_be_bytes([c[0], c[1]]);
        }
        g
    };
    // find longest run of zero groups (length >= 2)
    let (mut best_start, mut best_len) = (usize::MAX, 0usize);
    let mut i = 0;
    while i < 8 {
        if groups[i] == 0 {
            let start = i;
            while i < 8 && groups[i] == 0 {
                i += 1;
            }
            if i - start > best_len {
                best_len = i - start;
                best_start = start;
            }
        } else {
            i += 1;
        }
    }
    if best_len < 2 {
        return groups.iter().map(|g| format!("{g:x}")).collect::<Vec<_>>().join(":");
    }
    let head = groups[..best_start].iter().map(|g| format!("{g:x}")).collect::<Vec<_>>().join(":");
    let tail = groups[best_start + best_len..]
        .iter()
        .map(|g| format!("{g:x}"))
        .collect::<Vec<_>>()
        .join(":");
    match (head.is_empty(), tail.is_empty()) {
        (true, true) => "::".into(),
        (true, false) => format!("::{tail}"),
        (false, true) => format!("{head}::"),
        (false, false) => format!("{head}::{tail}"),
    }
}

fn address_type(addr: u128) -> &'static str {
    let b = addr.to_be_bytes();
    if addr == u128::from(std::net::Ipv6Addr::UNSPECIFIED) {
        "Unspecified (::)"
    } else if addr == 1 {
        "Loopback (::1)"
    } else if b[0] == 0xff {
        "Multicast (ff00::/8)"
    } else if b[0] == 0xfe && (b[1] & 0xc0) == 0x80 {
        "Link-local (fe80::/10)"
    } else if b[0] == 0xfe && (b[1] & 0xc0) == 0xc0 {
        "Site-local (deprecated fec0::/10)"
    } else if b[0] & 0xfe == 0xfc {
        "Unique local (fc00::/7)"
    } else if b[..12] == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff] {
        "IPv4-mapped (::ffff:0:0/96)"
    } else if b[0] & 0xe0 == 0x20 {
        "Global unicast (2000::/3)"
    } else {
        "Other / reserved"
    }
}

fn binary_of(addr: u128) -> String {
    let b = addr.to_be_bytes();
    b.iter()
        .map(|x| format!("{x:08b}"))
        .collect::<Vec<_>>()
        .join(":")
}

pub fn calculate(input: &str) -> Result<Ipv6Info, String> {
    let (addr, prefix) = parse_with_prefix(input)?;
    let mask: u128 = if prefix == 0 {
        0
    } else {
        u128::MAX << (128 - prefix as u32)
    };
    let network = addr & mask;
    let total_str = if prefix == 0 {
        "2^128".to_string()
    } else {
        let total: u128 = 1u128 << (128 - prefix as u32);
        total.to_string()
    };
    Ok(Ipv6Info {
        input: input.trim().to_string(),
        compressed: compress(addr),
        expanded: expand(addr),
        prefix,
        network: compress(network),
        first_address: compress(network),
        last_address: compress(network | !mask),
        total_addresses: total_str,
        address_type: address_type(addr).to_string(),
        binary: binary_of(addr),
    })
}

/// Expand a possibly-compressed address to full form.
pub fn expand_input(input: &str) -> Result<String, String> {
    let (addr, _) = parse_with_prefix(input)?;
    Ok(expand(addr))
}

/// Produce the EUI-64 interface identifier for a MAC (with the U/L bit flip).
pub fn eui64(mac: &str) -> Result<String, String> {
    let clean: String = mac
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    if clean.len() != 12 {
        return Err(format!("Invalid MAC '{mac}' — expected 12 hex digits"));
    }
    let mut bytes = Vec::new();
    for i in (0..12).step_by(2) {
        bytes.push(u8::from_str_radix(&clean[i..i + 2], 16).map_err(|_| "bad hex".to_string())?);
    }
    bytes[0] ^= 0x02; // flip U/L bit
    Ok(format!("{:02x}{:02x}:{:02x}ff:fe{:02x}:{:02x}{:02x}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_rfc5952() {
        assert_eq!(compress(1), "::1");
        assert_eq!(compress(0), "::");
        let a = u128::from_be_bytes([0x20,0x01,0x0d,0xb8,0,0,0,0,0,0,0,0,0,0,0,0x01]);
        assert_eq!(compress(a), "2001:db8::1");
        // longest run wins: 2001:0:0:1:0:0:0:1 -> 2001:0:0:1::1
        let b = u128::from_be_bytes([0x20,0x01,0,0, 0,0, 0,1, 0,0, 0,0, 0,0, 0,0x01]);
        assert_eq!(compress(b), "2001:0:0:1::1");
    }

    #[test]
    fn calculate_full_prefix() {
        let info = calculate("2001:db8::1/64").unwrap();
        assert_eq!(info.compressed, "2001:db8::1");
        assert_eq!(info.expanded, "2001:0db8:0000:0000:0000:0000:0000:0001");
        assert_eq!(info.prefix, 64);
        assert_eq!(info.network, "2001:db8::");
        assert_eq!(info.first_address, "2001:db8::");
        assert_eq!(info.last_address, "2001:db8::ffff:ffff:ffff:ffff");
        assert_eq!(info.total_addresses, "18446744073709551616");
        assert_eq!(info.address_type, "Global unicast (2000::/3)");
    }

    #[test]
    fn types_and_edge_cases() {
        assert_eq!(calculate("::1").unwrap().address_type, "Loopback (::1)");
        assert_eq!(calculate("fe80::1").unwrap().address_type, "Link-local (fe80::/10)");
        assert_eq!(calculate("ff02::1").unwrap().address_type, "Multicast (ff00::/8)");
        assert_eq!(calculate("fd00::1").unwrap().address_type, "Unique local (fc00::/7)");
        assert_eq!(
            calculate("::ffff:192.168.1.1").unwrap().address_type,
            "IPv4-mapped (::ffff:0:0/96)"
        );
        assert_eq!(calculate("::/128").unwrap().address_type, "Unspecified (::)");
        assert!(calculate("2001:db8::/129").is_err());
        assert!(calculate("2001:db8::zz/64").is_err());
    }

    #[test]
    fn eui64_conversion() {
        assert_eq!(eui64("00:1a:2b:3c:4d:5e").unwrap(), "021a:2bff:fe3c:4d5e");
        assert_eq!(eui64("001a2b3c4d5e").unwrap(), "021a:2bff:fe3c:4d5e");
        assert!(eui64("00:1a").is_err());
    }

    #[test]
    fn expand_input_matches() {
        assert_eq!(expand_input("2001:db8::1").unwrap(), "2001:0db8:0000:0000:0000:0000:0000:0001");
    }
}

