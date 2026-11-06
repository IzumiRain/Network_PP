use serde::Serialize;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordType {
    A = 1,
    Ns = 2,
    Cname = 5,
    Soa = 6,
    Ptr = 12,
    Mx = 15,
    Txt = 16,
    Aaaa = 28,
    Srv = 33,
}

impl RecordType {
    fn from_u16(v: u16) -> Option<Self> {
        Some(match v {
            1 => RecordType::A,
            2 => RecordType::Ns,
            5 => RecordType::Cname,
            6 => RecordType::Soa,
            12 => RecordType::Ptr,
            15 => RecordType::Mx,
            16 => RecordType::Txt,
            28 => RecordType::Aaaa,
            33 => RecordType::Srv,
            _ => return None,
        })
    }
    fn name(self) -> &'static str {
        match self {
            RecordType::A => "A",
            RecordType::Ns => "NS",
            RecordType::Cname => "CNAME",
            RecordType::Soa => "SOA",
            RecordType::Ptr => "PTR",
            RecordType::Mx => "MX",
            RecordType::Txt => "TXT",
            RecordType::Aaaa => "AAAA",
            RecordType::Srv => "SRV",
        }
    }
}

pub fn parse_type_name(s: &str) -> Result<RecordType, String> {
    match s.trim().to_uppercase().as_str() {
        "A" => Ok(RecordType::A),
        "AAAA" => Ok(RecordType::Aaaa),
        "CNAME" => Ok(RecordType::Cname),
        "MX" => Ok(RecordType::Mx),
        "NS" => Ok(RecordType::Ns),
        "TXT" => Ok(RecordType::Txt),
        "SOA" => Ok(RecordType::Soa),
        "PTR" => Ok(RecordType::Ptr),
        "SRV" => Ok(RecordType::Srv),
        other => Err(format!(
            "unsupported record type '{other}' — A, AAAA, CNAME, MX, NS, TXT, SOA, PTR, SRV"
        )),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: String,
    pub ttl: u32,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsResult {
    pub server: String,
    pub query: String,
    pub qtype: String,
    pub ok: bool,
    pub rcode: String,
    pub answers: Vec<DnsRecord>,
    pub response_ms: u64,
    pub error: Option<String>,
}

// ---------- wire building ----------

fn encode_name(name: &str, out: &mut Vec<u8>) {
    for label in name.trim_end_matches('.').split('.') {
        if label.is_empty() {
            continue;
        }
        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }
    out.push(0);
}

pub fn build_query(id: u16, name: &str, qtype: RecordType) -> Vec<u8> {
    let mut q = Vec::with_capacity(64);
    q.extend_from_slice(&id.to_be_bytes());
    q.extend_from_slice(&0x0100u16.to_be_bytes()); // RD=1
    q.extend_from_slice(&1u16.to_be_bytes()); // QDCOUNT
    q.extend_from_slice(&0u16.to_be_bytes());
    q.extend_from_slice(&0u16.to_be_bytes());
    q.extend_from_slice(&0u16.to_be_bytes());
    encode_name(name, &mut q);
    q.extend_from_slice(&(qtype as u16).to_be_bytes());
    q.extend_from_slice(&1u16.to_be_bytes()); // IN
    q
}

// ---------- wire parsing ----------

struct Cursor<'a> {
    msg: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn u16(&mut self) -> Option<u16> {
        let v = u16::from_be_bytes([*self.msg.get(self.pos)?, *self.msg.get(self.pos + 1)?]);
        self.pos += 2;
        Some(v)
    }
    fn u32(&mut self) -> Option<u32> {
        let b = self.msg.get(self.pos..self.pos + 4)?;
        self.pos += 4;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn byte(&mut self) -> Option<u8> {
        let b = *self.msg.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.msg.get(self.pos..self.pos + n)?;
        self.pos += n;
        Some(s)
    }
    /// Read a (possibly compressed) domain name. Offsets are absolute.
    fn name(&mut self) -> Option<String> {
        let mut labels: Vec<String> = Vec::new();
        let mut jumps = 0;
        let mut pos = self.pos;
        let mut jumped = false;
        let mut next_after_jump = 0usize;
        loop {
            let len = *self.msg.get(pos)?;
            if len & 0xC0 == 0xC0 {
                if jumps > 16 {
                    return None;
                }
                let ptr = (((len & 0x3F) as usize) << 8) | (*self.msg.get(pos + 1)? as usize);
                if !jumped {
                    next_after_jump = pos + 2;
                    jumped = true;
                }
                pos = ptr;
                jumps += 1;
                continue;
            }
            if len == 0 {
                break;
            }
            let label = self.msg.get(pos + 1..pos + 1 + len as usize)?;
            labels.push(String::from_utf8_lossy(label).to_string());
            pos += 1 + len as usize;
        }
        if jumped {
            self.pos = next_after_jump;
        } else {
            self.pos = pos + 1;
        }
        Some(if labels.is_empty() {
            ".".into()
        } else {
            labels.join(".")
        })
    }
}

fn decode_rdata(cur: &mut Cursor, rtype: RecordType, rdlen: usize) -> Option<String> {
    match rtype {
        RecordType::A => {
            let b = cur.take(4)?;
            Some(b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("."))
        }
        RecordType::Aaaa => {
            let b = cur.take(16)?;
            Some(b.chunks(2).map(|c| format!("{:x}", u16::from_be_bytes([c[0], c[1]]))).collect::<Vec<_>>().join(":"))
        }
        RecordType::Cname | RecordType::Ns | RecordType::Ptr => cur.name(),
        RecordType::Mx => {
            let pref = cur.u16()?;
            let host = cur.name()?;
            Some(format!("{pref} {host}"))
        }
        RecordType::Txt => {
            let mut out = String::new();
            let end = cur.pos + rdlen;
            while cur.pos < end {
                let l = cur.byte()? as usize;
                let s = cur.take(l)?;
                out.push_str(&String::from_utf8_lossy(s));
            }
            Some(out)
        }
        RecordType::Srv => {
            let _prio = cur.u16()?;
            let _weight = cur.u16()?;
            let port = cur.u16()?;
            let target = cur.name()?;
            Some(format!("{port} {target}"))
        }
        RecordType::Soa => {
            let mname = cur.name()?;
            let rname = cur.name()?;
            let serial = cur.u32()?;
            let refresh = cur.u32()?;
            let retry = cur.u32()?;
            let expire = cur.u32()?;
            let minimum = cur.u32()?;
            Some(format!("{mname} {rname} serial={serial} refresh={refresh} retry={retry} expire={expire} min={minimum}"))
        }
    }
}

/// Parse a DNS response message into (rcode_name, records).
pub fn parse_response(msg: &[u8]) -> Result<(String, Vec<DnsRecord>), String> {
    if msg.len() < 12 {
        return Err("response shorter than DNS header".into());
    }
    let mut cur = Cursor { msg, pos: 0 };
    let _id = cur.u16().ok_or("short header")?;
    let flags = cur.u16().ok_or("short header")?;
    let qd = cur.u16().ok_or("short header")?;
    let an = cur.u16().ok_or("short header")?;
    cur.u16(); // NSCOUNT
    cur.u16(); // ARCOUNT

    let rcode = flags & 0x000F;
    let rcode_name = match rcode {
        0 => "NOERROR",
        1 => "FORMERR",
        2 => "SERVFAIL",
        3 => "NXDOMAIN",
        4 => "NOTIMP",
        5 => "REFUSED",
        _ => "UNKNOWN",
    }
    .to_string();

    for _ in 0..qd {
        cur.name();
        cur.u16();
        cur.u16();
    }

    let mut records = Vec::new();
    let total = an as usize;
    for _ in 0..total {
        let name = cur.name().ok_or("bad answer name")?;
        let rtype_v = cur.u16().ok_or("bad answer type")?;
        let _class = cur.u16();
        let ttl = cur.u32().ok_or("bad answer ttl")?;
        let rdlen = cur.u16().ok_or("bad answer rdlen")? as usize;
        let Some(rt) = RecordType::from_u16(rtype_v) else {
            cur.take(rdlen);
            continue;
        };
        let end = cur.pos + rdlen;
        let value = decode_rdata(&mut cur, rt, rdlen).ok_or("bad rdata")?;
        cur.pos = end;
        records.push(DnsRecord {
            name,
            rtype: rt.name().to_string(),
            ttl,
            value,
        });
    }
    Ok((rcode_name, records))
}

// ---------- client ----------

fn random_id() -> u16 {
    let mut b = [0u8; 2];
    let _ = getrandom::getrandom(&mut b);
    u16::from_be_bytes(b)
}

pub fn lookup(name: &str, qtype: RecordType, server: &str, timeout_ms: u64) -> DnsResult {
    let server = if server.contains(':') {
        server.to_string()
    } else {
        format!("{server}:53")
    };
    let id = random_id();
    let query = build_query(id, name, qtype);
    let t0 = Instant::now();

    let mut result = DnsResult {
        server: server.clone(),
        query: name.to_string(),
        qtype: qtype.name().to_string(),
        ok: false,
        rcode: "—".into(),
        answers: Vec::new(),
        response_ms: 0,
        error: None,
    };

    let run = || -> Result<(String, Vec<DnsRecord>), String> {
        let local = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        local.set_read_timeout(Some(Duration::from_millis(timeout_ms.max(100)))).map_err(|e| e.to_string())?;
        local
            .send_to(&query, &server)
            .map_err(|e| format!("send failed: {e}"))?;
        let mut buf = [0u8; 4096];
        let (n, _) = local
            .recv_from(&mut buf)
            .map_err(|e| format!("no answer in {timeout_ms} ms ({e})"))?;
        let (rcode, records) = parse_response(&buf[..n])?;
        // verify id + QR
        if buf.len() >= 2 && u16::from_be_bytes([buf[0], buf[1]]) != id {
            return Err("response id mismatch".into());
        }
        if buf.len() >= 3 && buf[2] & 0x80 == 0 {
            return Err("not a response (QR bit clear)".into());
        }
        Ok((rcode, records))
    };

    match run() {
        Ok((rcode, answers)) => {
            result.ok = rcode == "NOERROR";
            result.rcode = rcode;
            result.answers = answers;
        }
        Err(e) => result.error = Some(e),
    }
    result.response_ms = t0.elapsed().as_millis() as u64;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_wire_format() {
        let q = build_query(0x1234, "example.com", RecordType::A);
        assert_eq!(&q[0..2], &[0x12, 0x34]); // id
        assert_eq!(&q[2..4], &[0x01, 0x00]); // RD
        assert_eq!(&q[4..6], &[0, 1]); // QDCOUNT
        // name: 7 example 3 com 0 = bytes 12..23
        assert_eq!(q[12], 7);
        assert_eq!(&q[13..20], b"example");
        assert_eq!(q[20], 3);
        assert_eq!(&q[21..24], b"com");
        assert_eq!(q[24], 0);
        assert_eq!(&q[25..27], &[0, 1]); // QTYPE A
        assert_eq!(&q[27..29], &[0, 1]); // QCLASS IN
        assert_eq!(q.len(), 29);
    }

    #[test]
    fn parses_synthetic_a_response() {
        // Build a response: question example.com A + one A answer using a
        // compression pointer (0xC00C) for the answer name.
        let mut m = Vec::new();
        m.extend_from_slice(&0x1234u16.to_be_bytes());
        m.extend_from_slice(&0x8180u16.to_be_bytes()); // QR+RD+RA, NOERROR
        m.extend_from_slice(&1u16.to_be_bytes()); // QD
        m.extend_from_slice(&1u16.to_be_bytes()); // AN
        m.extend_from_slice(&0u16.to_be_bytes());
        m.extend_from_slice(&0u16.to_be_bytes());
        encode_name("example.com", &mut m);
        m.extend_from_slice(&1u16.to_be_bytes()); // A
        m.extend_from_slice(&1u16.to_be_bytes()); // IN
        // answer
        m.extend_from_slice(&[0xC0, 0x0C]); // pointer to QNAME at offset 12
        m.extend_from_slice(&1u16.to_be_bytes()); // A
        m.extend_from_slice(&1u16.to_be_bytes()); // IN
        m.extend_from_slice(&3600u32.to_be_bytes()); // TTL
        m.extend_from_slice(&4u16.to_be_bytes()); // RDLENGTH
        m.extend_from_slice(&[93, 184, 216, 34]);
        let (rcode, records) = parse_response(&m).unwrap();
        assert_eq!(rcode, "NOERROR");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "example.com");
        assert_eq!(records[0].rtype, "A");
        assert_eq!(records[0].ttl, 3600);
        assert_eq!(records[0].value, "93.184.216.34");
    }

    #[test]
    fn parses_nxdomain_without_answers() {
        let mut m = Vec::new();
        m.extend_from_slice(&0x0001u16.to_be_bytes());
        m.extend_from_slice(&0x8183u16.to_be_bytes()); // NXDOMAIN
        m.extend_from_slice(&1u16.to_be_bytes());
        m.extend_from_slice(&0u16.to_be_bytes());
        m.extend_from_slice(&0u16.to_be_bytes());
        m.extend_from_slice(&0u16.to_be_bytes());
        encode_name("nope.invalid", &mut m);
        m.extend_from_slice(&1u16.to_be_bytes());
        m.extend_from_slice(&1u16.to_be_bytes());
        let (rcode, records) = parse_response(&m).unwrap();
        assert_eq!(rcode, "NXDOMAIN");
        assert!(records.is_empty());
    }

    #[test]
    fn rejects_short_messages() {
        assert!(parse_response(&[0, 1, 2]).is_err());
        assert!(parse_response(&[]).is_err());
    }

    #[test]
    fn type_parsing() {
        assert_eq!(parse_type_name("a").unwrap(), RecordType::A);
        assert_eq!(parse_type_name("aaaa").unwrap(), RecordType::Aaaa);
        assert!(parse_type_name("BOGUS").is_err());
    }
}
