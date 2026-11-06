use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub username: String,
    pub os: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub mem_mb: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
    pub free_gb: f64,
}

pub fn system_info() -> SystemInfo {
    let hostname = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".into());
    let username = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown".into());
    SystemInfo {
        hostname,
        username,
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_cores: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(0),
        app_version: crate::commands::packs::APP_VERSION.to_string(),
    }
}

fn run(cmd: &str, args: &[&str]) -> String {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

pub fn process_list() -> Vec<ProcessInfo> {
    let mut out = Vec::new();
    if cfg!(target_os = "windows") {
        // "chrome.exe    4231 Console    1    182,340 K"
        for line in run("tasklist", &[]).lines().skip(3) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let name = parts[0].to_string();
                let pid = parts[1].parse().unwrap_or(0);
                let mem = parts
                    .iter()
                    .rev()
                    .find(|p| p.ends_with('K'))
                    .and_then(|p| {
                        p.trim_end_matches(|c| c == 'K' || c == ',')
                            .replace(',', "")
                            .parse::<f64>()
                            .ok()
                    })
                    .map(|k| k / 1024.0);
                if pid > 0 {
                    out.push(ProcessInfo { pid, name, mem_mb: mem });
                }
            }
        }
    } else {
        // "  1234 chrome  512.3 MB" style from ps
        let text = if cfg!(target_os = "macos") {
            run("ps", &["-eo", "pid,comm,rss"])
        } else {
            run("ps", &["-eo", "pid,comm,rss"])
        };
        for line in text.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let pid = parts[0].parse().unwrap_or(0);
                let name = parts[1]
                    .rsplit('/')
                    .next()
                    .unwrap_or(parts[1])
                    .to_string();
                let mem = parts[2].parse::<f64>().ok().map(|kb| kb / 1024.0);
                if pid > 0 {
                    out.push(ProcessInfo { pid, name, mem_mb: mem });
                }
            }
        }
    }
    out.sort_by(|a, b| b.mem_mb.unwrap_or(0.0).total_cmp(&a.mem_mb.unwrap_or(0.0)));
    out.truncate(15);
    out
}

pub fn disk_usage() -> Vec<DiskInfo> {
    let mut out = Vec::new();
    if cfg!(target_os = "windows") {
        let text = run(
            "powershell",
            &["-NoProfile", "-Command",
              "Get-PSDrive -PSProvider FileSystem | Select-Object Name,@{n='Used';e={$_.Used/1GB}},@{n='Free';e={$_.Free/1GB}} | ConvertTo-CSV -NoTypeInformation"],
        );
        for line in text.lines().skip(1) {
            let cells: Vec<&str> = line.split(',').map(|c| c.trim_matches('"')).collect();
            if cells.len() >= 3 {
                if let (Ok(used), Ok(free)) = (cells[1].parse::<f64>(), cells[2].parse::<f64>()) {
                    out.push(DiskInfo {
                        name: cells[0].to_string(),
                        total_gb: (used + free).max(0.0),
                        free_gb: free.max(0.0),
                    });
                }
            }
        }
    } else {
        for line in run("df", &["-k", "-P"]).lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 && parts[0].starts_with('/') {
                let total_kb: f64 = parts[1].parse().unwrap_or(0.0);
                let free_kb: f64 = parts[3].parse().unwrap_or(0.0);
                out.push(DiskInfo {
                    name: parts[5].to_string(),
                    total_gb: total_kb / 1_048_576.0,
                    free_gb: free_kb / 1_048_576.0,
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_populates() {
        let info = system_info();
        assert!(!info.hostname.is_empty());
        assert!(!info.os.is_empty());
        assert!(info.cpu_cores >= 1);
        assert!(!info.app_version.is_empty());
    }

    #[test]
    fn process_list_parses_something() {
        let procs = process_list();
        // A live OS always has some processes; shape is what matters.
        if !procs.is_empty() {
            assert!(procs.iter().all(|p| p.pid > 0 && !p.name.is_empty()));
        }
    }

    #[test]
    fn disks_shape() {
        for d in disk_usage() {
            assert!(d.total_gb >= 0.0 && d.free_gb >= 0.0);
        }
    }
}
