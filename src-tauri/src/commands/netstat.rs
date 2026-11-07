use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Proto {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize)]
pub struct Connection {
    pub proto: Proto,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: Option<u16>,
    pub state: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetstatSnapshot {
    pub connections: Vec<Connection>,
    pub total: usize,
    pub by_state: Vec<(String, usize)>,
}

pub fn snapshot() -> Result<NetstatSnapshot, String> {
    if cfg!(target_os = "windows") {
        let tcp = run("netstat", &["-ano", "-p", "tcp"]);
        let udp = run("netstat", &["-ano", "-p", "udp"]);
        let mut connections = Vec::new();
        if let Ok(t) = &tcp {
            connections.extend(parse_windows_netstat(t, Proto::Tcp));
        }
        if let Ok(u) = &udp {
            connections.extend(parse_windows_netstat(u, Proto::Udp));
        }
        Ok(finish(connections))
    } else {
        // ss is the modern tool; netstat is the fallback. -p needs privileges, try anyway.
        let out = run("ss", &["-H", "-tunap"]).or_else(|_| run("netstat", &["-tunp"]));
        let text = out?;
        Ok(finish(parse_linux_ss(&text)))
    }
}

fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = super::procutil::command(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Could not run {cmd}: {e}"))?;
    if !output.status.success() && output.stdout.is_empty() {
        return Err(format!("{cmd} exited with {}", output.status));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn finish(mut connections: Vec<Connection>) -> NetstatSnapshot {
    let mut by_state: std::collections::BTreeMap<String, usize> = Default::default();
    for c in &connections {
        *by_state.entry(c.state.clone()).or_default() += 1;
    }
    connections.sort_by(|a, b| {
        (a.proto as u8, &a.local_addr, a.local_port).cmp(&(b.proto as u8, &b.local_addr, b.local_port))
    });
    let total = connections.len();
    NetstatSnapshot {
        total,
        by_state: by_state.into_iter().collect(),
        connections,
    }
}

pub fn parse_windows_netstat(text: &str, proto: Proto) -> Vec<Connection> {
    let mut out = Vec::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // "  TCP    0.0.0.0:135    0.0.0.0:0    LISTENING    1234"
        // "  UDP    0.0.0.0:5353    *:*    4321"
        if parts.len() < 3 || parts[0].to_uppercase() != "TCP" && parts[0].to_uppercase() != "UDP" {
            continue;
        }
        if parts[0].to_uppercase() != if proto == Proto::Tcp { "TCP" } else { "UDP" } {
            continue;
        }
        let parse_ep = |s: &str| -> (String, Option<u16>) {
            match s.rsplit_once(':') {
                Some((a, p)) => (a.trim_matches(['[', ']']).to_string(), p.parse().ok()),
                None => (s.to_string(), None),
            }
        };
        let (local_addr, local_port) = parse_ep(parts[1]);
        let (remote_addr, remote_port) = parse_ep(parts[2]);
        if local_port.is_none() {
            continue;
        }
        let state = match proto {
            Proto::Tcp => parts.get(3).copied().unwrap_or("UNKNOWN").to_string(),
            Proto::Udp => "—".into(),
        };
        let pid = match proto {
            Proto::Tcp => parts.get(4).and_then(|s| s.parse().ok()),
            Proto::Udp => parts.get(3).and_then(|s| s.parse().ok()),
        };
        out.push(Connection {
            proto,
            local_addr,
            local_port: local_port.unwrap_or(0),
            remote_addr,
            remote_port,
            state,
            pid,
        });
    }
    out
}

pub fn parse_linux_ss(text: &str) -> Vec<Connection> {
    let mut out = Vec::new();
    for line in text.lines() {
        // ss -H -tunap:
        // "tcp   ESTAB 0      0    192.168.1.10:51000   93.184.216.34:443   users:(("chrome",pid=1234))"
        // "udp   UNCONN 0    0    0.0.0.0:5353   0.0.0.0:*"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }
        let proto = match parts[0] {
            "tcp" => Proto::Tcp,
            "udp" => Proto::Udp,
            _ => continue,
        };
        let (state, rest) = if proto == Proto::Tcp {
            (parts[1].to_uppercase(), &parts[3..])
        } else {
            ("UNCONN".to_string(), &parts[2..])
        };
        let parse_ep = |s: &str| -> (String, Option<u16>) {
            match s.rsplit_once(':') {
                Some((a, p)) => (a.trim_start_matches(['[', '*']).to_string(), p.parse().ok()),
                None => (s.to_string(), None),
            }
        };
        // find two endpoint-looking tokens: "addr:port"
        let mut eps: Vec<(String, Option<u16>)> = Vec::new();
        for p in rest {
            if p.contains(':') && !p.starts_with("users:") && eps.len() < 2 {
                let (a, b) = parse_ep(p);
                if b.is_some() || a == "*" || a == "0.0.0.0" {
                    eps.push((a, b));
                }
            }
        }
        if eps.len() < 2 {
            continue;
        }
        let pid = line
            .split("pid=")
            .nth(1)
            .and_then(|s| s.split([',', ')']).next())
            .and_then(|s| s.parse().ok());
        out.push(Connection {
            proto,
            local_addr: eps[0].0.clone(),
            local_port: eps[0].1.unwrap_or(0),
            remote_addr: eps[1].0.clone(),
            remote_port: eps[1].1,
            state,
            pid,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIN_TCP: &str = "\nActive Connections\n\n  Proto  Local Address          Foreign Address        State           PID\n  TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1234\n  TCP    192.168.1.10:51000     93.184.216.34:443      ESTABLISHED     5678\n  TCP    [::1]:1900             [::]:0                 LISTENING       999\n";

    #[test]
    fn parses_windows_tcp() {
        let rows = parse_windows_netstat(WIN_TCP, Proto::Tcp);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].local_port, 135);
        assert_eq!(rows[0].state, "LISTENING");
        assert_eq!(rows[0].pid, Some(1234));
        assert_eq!(rows[1].remote_addr, "93.184.216.34");
        assert_eq!(rows[1].remote_port, Some(443));
        assert_eq!(rows[2].local_addr, "::1");
    }

    #[test]
    fn parses_windows_udp() {
        let text = "  UDP    0.0.0.0:5353           *:*                                    4321\n  UDP    [::]:1900              *:*                                    999\n";
        let rows = parse_windows_netstat(text, Proto::Udp);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].local_port, 5353);
        assert_eq!(rows[0].state, "—");
        assert_eq!(rows[0].pid, Some(4321));
        assert_eq!(rows[0].remote_port, None);
    }

    #[test]
    fn parses_linux_ss_output() {
        let text = "tcp   ESTAB 0      0      192.168.1.10:51000   93.184.216.34:443       users:((\"chrome\",pid=4242,fd=99))\nudp   UNCONN 0     0      0.0.0.0:5353         0.0.0.0:*\ntcp   LISTEN 0     128    0.0.0.0:22           0.0.0.0:*               users:((\"sshd\",pid=800,fd=3))\n";
        let rows = parse_linux_ss(text);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].state, "ESTAB");
        assert_eq!(rows[0].local_port, 51000);
        assert_eq!(rows[0].remote_port, Some(443));
        assert_eq!(rows[0].pid, Some(4242));
        assert_eq!(rows[1].proto, Proto::Udp);
        assert_eq!(rows[1].local_port, 5353);
        assert_eq!(rows[2].state, "LISTEN");
        assert_eq!(rows[2].pid, Some(800));
    }

    #[test]
    fn ignores_garbage_lines() {
        assert!(parse_windows_netstat("hello world\n\n", Proto::Tcp).is_empty());
        assert!(parse_linux_ss("not a socket line\n").is_empty());
    }
}
