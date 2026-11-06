// 0.4 diagnostics commands: DNS lookups, HTTP tests, system info, sessions.

use crate::commands::{dns, httptest, sessions::SessionRow, sysinfo};
use crate::commands::store::Store;

#[tauri::command]
pub fn dns_lookup(
    name: String,
    qtype: String,
    server: String,
    timeout_ms: u64,
) -> dns::DnsResult {
    let rt = match dns::parse_type_name(&qtype) {
        Ok(rt) => rt,
        Err(e) => {
            return dns::DnsResult {
                server,
                query: name,
                qtype,
                ok: false,
                rcode: "—".into(),
                answers: Vec::new(),
                response_ms: 0,
                error: Some(e),
            }
        }
    };
    dns::lookup(&name, rt, &server, timeout_ms)
}

#[tauri::command]
pub fn dns_compare(
    name: String,
    qtype: String,
    servers: String,
    timeout_ms: u64,
) -> Vec<dns::DnsResult> {
    let rt = match dns::parse_type_name(&qtype) {
        Ok(rt) => rt,
        Err(_) => dns::RecordType::A,
    };
    servers
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| dns::lookup(&name, rt, s, timeout_ms))
        .collect()
}

#[tauri::command]
pub fn http_test_cmd(
    url: String,
    method: String,
    timeout_ms: u64,
    proxy: Option<String>,
    store: tauri::State<'_, Store>,
) -> httptest::HttpTestResult {
    let proxy = proxy.or_else(|| store.setting_get("network.proxy"));
    httptest::http_test(&url, &method, timeout_ms, proxy.as_deref())
}

#[tauri::command]
pub fn sys_info_cmd() -> sysinfo::SystemInfo {
    sysinfo::system_info()
}

#[tauri::command]
pub fn sys_processes() -> Vec<sysinfo::ProcessInfo> {
    sysinfo::process_list()
}

#[tauri::command]
pub fn sys_disks() -> Vec<sysinfo::DiskInfo> {
    sysinfo::disk_usage()
}

#[tauri::command]
pub fn session_create(title: String, store: tauri::State<'_, Store>) -> Result<i64, String> {
    store.session_create(&title)
}

#[tauri::command]
pub fn session_list(store: tauri::State<'_, Store>) -> Result<Vec<SessionRow>, String> {
    store.session_list()
}

#[tauri::command]
pub fn session_add_result(
    session_id: i64,
    kind: String,
    target: String,
    ok: bool,
    summary: String,
    store: tauri::State<'_, Store>,
) -> Result<(), String> {
    store.session_add_result(session_id, &kind, &target, ok, &summary)
}

#[tauri::command]
pub fn session_results(
    session_id: i64,
    store: tauri::State<'_, Store>,
) -> Result<Vec<crate::commands::sessions::SessionResultRow>, String> {
    store.session_results(session_id)
}

#[tauri::command]
pub fn session_set_notes(
    session_id: i64,
    notes: String,
    store: tauri::State<'_, Store>,
) -> Result<(), String> {
    store.session_set_notes(session_id, &notes)
}

#[tauri::command]
pub fn session_delete(session_id: i64, store: tauri::State<'_, Store>) -> Result<(), String> {
    store.session_delete(session_id)
}

#[tauri::command]
pub fn save_text_file(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| format!("could not write {path}: {e}"))
}

#[tauri::command]
pub fn session_export_md(
    session_id: i64,
    store: tauri::State<'_, Store>,
) -> Result<String, String> {
    store.session_export_md(session_id)
}
