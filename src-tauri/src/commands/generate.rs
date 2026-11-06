use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Clone, Serialize)]
pub struct Generated {
    pub value: String,
    pub count: usize,
    pub extra: Option<String>,
}

fn secure_bytes(n: usize) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; n];
    getrandom::getrandom(&mut buf).map_err(|e| format!("entropy unavailable: {e}"))?;
    Ok(buf)
}

pub fn uuid_v4(count: usize) -> Result<Generated, String> {
    let count = count.clamp(1, 50);
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(uuid::Uuid::new_v4().to_string());
    }
    Ok(Generated {
        value: values.join("\n"),
        count,
        extra: None,
    })
}

pub fn mac_addresses(count: usize) -> Result<Generated, String> {
    let count = count.clamp(1, 50);
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let b = secure_bytes(6)?;
        // locally administered + unicast: set U/L bit, clear multicast bit
        values.push(format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            (b[0] | 0x02) & 0xfe,
            b[1], b[2], b[3], b[4], b[5]
        ));
    }
    Ok(Generated {
        value: values.join("\n"),
        count,
        extra: Some("locally administered (x2:xx), unicast".into()),
    })
}

pub fn ipv4_addresses(count: usize, private_only: bool) -> Result<Generated, String> {
    let count = count.clamp(1, 50);
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let b = secure_bytes(4)?;
        let (o2, o3, o4) = (b[1], b[2], b[3]);
        let ip = if private_only {
            // draw inside RFC1918 pools
            match b[0] % 3 {
                0 => format!("10.{o2}.{o3}.{o4}"),
                1 => format!("172.{}.{}.{}", 16 + (o2 % 16), o3, o4),
                _ => format!("192.168.{o2}.{o3}"),
            }
        } else {
            let o1 = b[0].clamp(1, 223); // avoid 0, loopback-ish 127 handled below, multicast+
            if o1 == 127 || o1 >= 224 {
                format!("93.184.{o3}.{o4}")
            } else {
                format!("{o1}.{o2}.{o3}.{o4}")
            }
        };
        values.push(ip);
    }
    Ok(Generated {
        value: values.join("\n"),
        count,
        extra: if private_only { Some("RFC 1918 ranges".into()) } else { None },
    })
}

const PW_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const PW_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const PW_DIGITS: &[u8] = b"0123456789";
const PW_SYMBOLS: &[u8] = b"!@#$%^&*-_=+?";

pub fn password(length: usize, symbols: bool) -> Result<Generated, String> {
    let length = length.clamp(8, 128);
    let mut pool: Vec<u8> = Vec::new();
    pool.extend_from_slice(PW_LOWER);
    pool.extend_from_slice(PW_UPPER);
    pool.extend_from_slice(PW_DIGITS);
    if symbols {
        pool.extend_from_slice(PW_SYMBOLS);
    }
    // guarantee one of each class, then fill randomly
    let entropy = secure_bytes(length * 2)?;
    let pick = |b: u8, set: &[u8]| set[b as usize % set.len()] as char;
    let mut chars: Vec<char> = vec![
        pick(entropy[0], PW_LOWER),
        pick(entropy[1], PW_UPPER),
        pick(entropy[2], PW_DIGITS),
    ];
    if symbols {
        chars.push(pick(entropy[3], PW_SYMBOLS));
    }
    for b in &entropy[chars.len()..length] {
        chars.push(pick(*b, &pool));
    }
    // shuffle with entropy
    for i in (1..chars.len()).rev() {
        let j = (entropy[i] as usize) % (i + 1);
        chars.swap(i, j);
    }
    Ok(Generated {
        value: chars.into_iter().collect(),
        count: 1,
        extra: Some(format!("{length} characters, mixed classes")),
    })
}

pub fn hash_text(text: &str) -> Generated {
    let mut h256 = Sha256::new();
    h256.update(text.as_bytes());
    let d256 = h256.finalize();
    let mut h512 = Sha512::new();
    h512.update(text.as_bytes());
    let d512 = h512.finalize();
    let hex = |d: &[u8]| d.iter().map(|b| format!("{b:02x}")).collect::<String>();
    Generated {
        value: hex(&d256),
        count: 1,
        extra: Some(hex(&d512)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_shape() {
        let g = uuid_v4(3).unwrap();
        let lines: Vec<&str> = g.value.lines().collect();
        assert_eq!(lines.len(), 3);
        for l in lines {
            assert_eq!(l.len(), 36);
            assert_eq!(l.chars().filter(|c| *c == '-').count(), 4);
            assert!(l.chars().nth(14) == Some('4')); // version nibble
        }
    }

    #[test]
    fn mac_shape_and_flags() {
        let g = mac_addresses(5).unwrap();
        assert_eq!(g.value.lines().count(), 5);
        for l in g.value.lines() {
            let first = u8::from_str_radix(&l[..2], 16).unwrap();
            assert_eq!(first & 0b11, 0b10); // locally administered, unicast
        }
    }

    #[test]
    fn ipv4_ranges() {
        let g = ipv4_addresses(20, true).unwrap();
        for l in g.value.lines() {
            let o: Vec<u32> = l.split('.').map(|x| x.parse().unwrap()).collect();
            let ok = o[0] == 10
                || (o[0] == 172 && (16..=31).contains(&o[1]))
                || (o[0] == 192 && o[1] == 168);
            assert!(ok, "bad private ip {l}");
        }
        let g = ipv4_addresses(20, false).unwrap();
        for l in g.value.lines() {
            let o1: u32 = l.split('.').next().unwrap().parse().unwrap();
            assert!((1..=223).contains(&o1) && o1 != 127);
        }
    }

    #[test]
    fn password_classes_and_length() {
        // below minimum clamps up to 8
        assert_eq!(password(4, false).unwrap().value.chars().count(), 8);
        for len in [8usize, 16, 32, 128] {
            let g = password(len, true).unwrap();
            assert_eq!(g.value.chars().count(), len);
            assert!(g.value.chars().any(|c| c.is_lowercase()));
            assert!(g.value.chars().any(|c| c.is_uppercase()));
            assert!(g.value.chars().any(|c| c.is_ascii_digit()));
            assert!(g.value.chars().any(|c| PW_SYMBOLS.contains(&(c as u8))));
        }
    }

    #[test]
    fn hashes_known_vectors() {
        let g = hash_text("hello");
        assert_eq!(
            g.value,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(g.extra.as_ref().unwrap().len(), 128);
    }
}
