use serde::Serialize;

// ---------- number systems ----------

#[derive(Debug, Clone, Serialize)]
pub struct NumberResult {
    pub decimal: String,
    pub binary: String,
    pub hex: String,
    pub octal: String,
    pub bits: usize,
}

pub fn convert_number(input: &str, base: u32) -> Result<NumberResult, String> {
    let trimmed = input.trim().replace(['_', ' '], "");
    let (digits, radix) = match base {
        2 | 8 | 10 | 16 => (trimmed.trim_start_matches("0x").trim_start_matches("0b"), base),
        _ => return Err("Base must be 2, 8, 10 or 16".into()),
    };
    let value = u128::from_str_radix(digits, radix)
        .map_err(|_| format!("'{input}' is not a valid base-{radix} number"))?;
    Ok(NumberResult {
        decimal: value.to_string(),
        binary: format!("{value:b}"),
        hex: format!("{value:x}"),
        octal: format!("{value:o}"),
        bits: 128 - value.leading_zeros() as usize,
    })
}

// ---------- IPv4 converters ----------

#[derive(Debug, Clone, Serialize)]
pub struct Ipv4Forms {
    pub dotted: String,
    pub integer: String,
    pub binary: String,
    pub hex: String,
}

pub fn ipv4_forms(input: &str) -> Result<Ipv4Forms, String> {
    let trimmed = input.trim();
    // accept dotted quad or raw integer
    let addr = if trimmed.contains('.') {
        crate::commands::cidr::parse_ipv4(trimmed)
            .ok_or_else(|| format!("Invalid IPv4 address: {trimmed}"))?
    } else {
        let v: u32 = trimmed
            .parse()
            .map_err(|_| format!("'{trimmed}' is not a valid 32-bit integer or dotted quad"))?;
        v
    };
    let octets = addr.to_be_bytes();
    Ok(Ipv4Forms {
        dotted: crate::commands::cidr::ipv4_string(addr),
        integer: addr.to_string(),
        binary: octets
            .iter()
            .map(|o| format!("{o:08b}"))
            .collect::<Vec<_>>()
            .join("."),
        hex: octets.iter().map(|o| format!("{o:02X}")).collect(),
    })
}

// ---------- MAC converters ----------

#[derive(Debug, Clone, Serialize)]
pub struct MacForms {
    pub normalized: String,
    pub windows: String,
    pub cisco: String,
    pub bare: String,
    pub oui: String,
    pub nic: String,
}

fn mac_bytes(input: &str) -> Result<[u8; 6], String> {
    let clean: String = input
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    if clean.len() != 12 {
        return Err(format!("'{input}' is not a MAC — need 12 hex digits"));
    }
    let mut out = [0u8; 6];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).map_err(|_| "bad hex".to_string())?;
    }
    Ok(out)
}

pub fn mac_forms(input: &str) -> Result<MacForms, String> {
    let b = mac_bytes(input)?;
    Ok(MacForms {
        normalized: b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(":"),
        windows: b.iter().map(|x| format!("{x:02X}")).collect::<Vec<_>>().join("-"),
        cisco: b
            .chunks(2)
            .map(|c| format!("{:02x}{:02x}", c[0], c[1]))
            .collect::<Vec<_>>()
            .join("."),
        bare: b.iter().map(|x| format!("{x:02X}")).collect(),
        oui: format!("{:02x}:{:02x}:{:02x}", b[0], b[1], b[2]),
        nic: format!("{:02x}:{:02x}:{:02x}", b[3], b[4], b[5]),
    })
}

// ---------- base64 / url / hex text ----------

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity((data.len() + 2).div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(B64[(n >> 18) as usize & 63] as char);
        out.push(B64[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { B64[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { B64[n as usize & 63] as char } else { '=' });
    }
    out
}

pub fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    let clean: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.is_empty() {
        return Ok(Vec::new());
    }
    let val = |c: char| -> Result<u32, String> {
        match c {
            'A'..='Z' => Ok(c as u32 - 'A' as u32),
            'a'..='z' => Ok(c as u32 - 'a' as u32 + 26),
            '0'..='9' => Ok(c as u32 - '0' as u32 + 52),
            '+' => Ok(62),
            '/' => Ok(63),
            '=' => Ok(0),
            _ => Err(format!("invalid base64 character '{c}'")),
        }
    };
    if clean.len() % 4 != 0 {
        return Err("base64 length must be a multiple of 4".into());
    }
    let mut out = Vec::with_capacity(clean.len() / 4 * 3);
    for chunk in clean.chunks(4) {
        let pad = chunk.iter().filter(|&&c| c == '=').count();
        let mut n: u32 = 0;
        for c in chunk {
            n = (n << 6) | val(*c)?;
        }
        out.push((n >> 16) as u8);
        if pad < 2 {
            out.push((n >> 8) as u8);
        }
        if pad < 1 {
            out.push(n as u8);
        }
    }
    Ok(out)
}

pub fn url_encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn url_decode(text: &str) -> Result<String, String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() + 1 && i + 2 < bytes.len() + 1 => {
                if i + 2 >= bytes.len() {
                    return Err("truncated % escape".into());
                }
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).map_err(|e| e.to_string())?;
                let v = u8::from_str_radix(hex, 16).map_err(|_| format!("bad % escape %{hex}"))?;
                out.push(v);
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|_| "result is not valid UTF-8".into())
}

pub fn text_to_hex(text: &str) -> String {
    text.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

pub fn hex_to_text(hex: &str) -> Result<String, String> {
    let clean: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() % 2 != 0 {
        return Err("hex length must be even".into());
    }
    let mut bytes = Vec::with_capacity(clean.len() / 2);
    for i in (0..clean.len()).step_by(2) {
        bytes.push(
            u8::from_str_radix(&clean[i..i + 2], 16).map_err(|_| format!("bad hex at {i}"))?,
        );
    }
    String::from_utf8(bytes).map_err(|_| "not valid UTF-8".into())
}

// ---------- timestamps ----------

#[derive(Debug, Clone, Serialize)]
pub struct TimestampResult {
    pub unix: i64,
    pub iso_utc: String,
    pub iso_local_hint: String,
}

/// Convert Unix seconds to UTC ISO-8601 (proleptic Gregorian, civil math).
pub fn timestamp_from_unix(unix: i64) -> TimestampResult {
    let days = unix.div_euclid(86_400);
    let secs_of_day = unix.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let hh = secs_of_day / 3600;
    let mm = (secs_of_day % 3600) / 60;
    let ss = secs_of_day % 60;
    TimestampResult {
        unix,
        iso_utc: format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z"),
        iso_local_hint: "shown in UTC — add your local offset".into(),
    }
}

pub fn timestamp_parse(iso: &str) -> Result<TimestampResult, String> {
    let s = iso.trim();
    let bytes = s.as_bytes();
    if bytes.len() < 19 {
        return Err("expected YYYY-MM-DD HH:MM:SS (T or space, Z optional)".into());
    }
    let num = |a: usize, b: usize| -> Result<i64, String> {
        s.get(a..b)
            .and_then(|x| x.parse::<i64>().ok())
            .ok_or_else(|| format!("bad number at {a}..{b}"))
    };
    let (y, mo, d) = (num(0, 4)?, num(5, 7)?, num(8, 10)?);
    let (hh, mm, ss) = (num(11, 13)?, num(14, 16)?, num(17, 19)?);
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || hh > 23 || mm > 59 || ss > 60 {
        return Err("date/time component out of range".into());
    }
    let days = days_from_civil(y, mo as u32, d as u32);
    let unix = days * 86_400 + hh * 3600 + mm * 60 + ss;
    Ok(timestamp_from_unix(unix))
}

/// Howard Hinnant's civil date algorithms.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = ((m + 9) % 12) as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_systems() {
        let r = convert_number("255", 10).unwrap();
        assert_eq!(r.binary, "11111111");
        assert_eq!(r.hex, "ff");
        assert_eq!(r.octal, "377");
        assert_eq!(r.bits, 8);
        let r = convert_number("0b1010", 2).unwrap();
        assert_eq!(r.decimal, "10");
        let r = convert_number("0xdeadbeef", 16).unwrap();
        assert_eq!(r.decimal, "3735928559");
        assert!(convert_number("129", 8).is_err());
        assert!(convert_number("xyz", 16).is_err());
    }

    #[test]
    fn ipv4_all_forms() {
        let r = ipv4_forms("192.168.1.1").unwrap();
        assert_eq!(r.integer, "3232235777");
        assert_eq!(r.binary, "11000000.10101000.00000001.00000001");
        assert_eq!(r.hex, "C0A80101");
        let r = ipv4_forms("3232235777").unwrap();
        assert_eq!(r.dotted, "192.168.1.1");
    }

    #[test]
    fn mac_all_forms() {
        let r = mac_forms("00:1A:2B:3C:4D:5E").unwrap();
        assert_eq!(r.normalized, "00:1a:2b:3c:4d:5e");
        assert_eq!(r.windows, "00-1A-2B-3C-4D-5E");
        assert_eq!(r.cisco, "001a.2b3c.4d5e");
        assert_eq!(r.bare, "001A2B3C4D5E");
        assert_eq!(r.oui, "00:1a:2b");
        assert!(mac_forms("00:1A").is_err());
    }

    #[test]
    fn base64_roundtrip() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b"hi!"), "aGkh");
        assert_eq!(base64_encode(b"ab"), "YWI=");
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("aGVsbG8=  ").unwrap(), b"hello");
        assert!(base64_decode("abc").is_err());
        assert!(base64_decode("ab!c").is_err());
        // random-ish roundtrip
        let data: Vec<u8> = (0..255u8).collect();
        assert_eq!(base64_decode(&base64_encode(&data)).unwrap(), data);
    }

    #[test]
    fn url_encoding() {
        assert_eq!(url_encode("a b&c=1"), "a%20b%26c%3D1");
        assert_eq!(url_decode("a%20b%26c%3D1").unwrap(), "a b&c=1");
        assert_eq!(url_decode("a+b").unwrap(), "a b");
        assert!(url_decode("%zz").is_err());
        assert!(url_decode("%2").is_err());
    }

    #[test]
    fn hex_text_roundtrip() {
        assert_eq!(text_to_hex("AB"), "4142");
        assert_eq!(hex_to_text("4142").unwrap(), "AB");
        assert!(hex_to_text("41 4").is_err());
    }

    #[test]
    fn timestamps_known_values() {
        let r = timestamp_from_unix(0);
        assert_eq!(r.iso_utc, "1970-01-01T00:00:00Z");
        let r = timestamp_from_unix(1_700_000_000);
        assert_eq!(r.iso_utc, "2023-11-14T22:13:20Z");
        let r = timestamp_parse("2023-11-14 22:13:20").unwrap();
        assert_eq!(r.unix, 1_700_000_000);
        let r = timestamp_parse("2024-02-29T12:00:00Z").unwrap();
        assert_eq!(r.unix, 1_709_208_000);
        assert!(timestamp_parse("2023-13-01 00:00:00").is_err());
        assert!(timestamp_parse("garbage").is_err());
    }
}
