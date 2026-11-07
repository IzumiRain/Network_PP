mod commands;
mod toolcmds;
mod diagcmds;
mod packetcaps;
mod updatecmds;

use commands::{cidr, content, netstat, packs, packets, ports, scan, store::{self, Store}};
use tauri::Manager;

pub const DEFAULT_REGISTRY_URL: &str =
    "https://cdn.jsdelivr.net/gh/networkpp/addons@main/index.json";

// ---------- calculators ----------

#[tauri::command]
fn cidr_calculate(input: String) -> Result<cidr::CidrInfo, String> {
    cidr::calculate(&input)
}

#[tauri::command]
fn cidr_split(input: String, new_prefix: u8, limit: usize) -> Result<Vec<String>, String> {
    cidr::split_subnet(&input, new_prefix, limit)
}

#[tauri::command]
fn cidr_range_to_cidrs(start: String, end: String) -> Result<Vec<String>, String> {
    cidr::range_to_cidrs(&start, &end)
}

#[tauri::command]
fn cidr_contains(cidr_input: String, ip: String) -> Result<bool, String> {
    cidr::contains(&cidr_input, &ip)
}

#[tauri::command]
fn port_lookup(port: u32) -> Result<ports::PortReport, String> {
    ports::lookup(port)
}

#[tauri::command]
fn port_search(query: String) -> Vec<ports::PortInfo> {
    ports::search(&query)
}

#[tauri::command]
fn port_describe_range(from: u32, to: u32) -> Result<String, String> {
    ports::describe_range(from, to)
}

// ---------- packets ----------

#[tauri::command]
fn packet_parse(bytes_hex: String) -> Result<packets::Packet, String> {
    let bytes = decode_hex(&bytes_hex)?;
    packets::parse(&bytes)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn packet_build_tcp(
    src: String,
    dst: String,
    sport: u16,
    dport: u16,
    fin: bool,
    syn: bool,
    rst: bool,
    psh: bool,
    ack: bool,
    urg: bool,
    payload: String,
) -> Result<packets::Packet, String> {
    let bytes = packets::build_ipv4_tcp_packet(
        &src,
        &dst,
        sport,
        dport,
        packets::TcpFlags {
            fin,
            syn,
            rst,
            psh,
            ack,
            urg,
            ..Default::default()
        },
        payload.as_bytes(),
    )?;
    packets::parse(&bytes)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn packet_build_tcp_hex(
    src: String,
    dst: String,
    sport: u16,
    dport: u16,
    fin: bool,
    syn: bool,
    rst: bool,
    psh: bool,
    ack: bool,
    urg: bool,
    payload: String,
) -> Result<String, String> {
    let bytes = packets::build_ipv4_tcp_packet(
        &src,
        &dst,
        sport,
        dport,
        packets::TcpFlags {
            fin,
            syn,
            rst,
            psh,
            ack,
            urg,
            ..Default::default()
        },
        payload.as_bytes(),
    )?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

#[tauri::command]
fn packet_hexdump(bytes_hex: String) -> Result<String, String> {
    let bytes = decode_hex(&bytes_hex)?;
    Ok(packets::hex_dump(&bytes))
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    let clean: String = s.chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    if clean.is_empty() {
        return Ok(Vec::new());
    }
    if clean.len() % 2 != 0 {
        return Err("Hex string must have an even number of digits".into());
    }
    (0..clean.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&clean[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex at position {i}"))
        })
        .collect()
}

// ---------- diagnostics ----------

#[tauri::command]
async fn scan_tcp(
    host: String,
    port_spec: String,
    timeout_ms: u64,
    with_banner: bool,
    max_threads: usize,
) -> Result<Vec<scan::PortScanResult>, String> {
    tauri::async_runtime::spawn_blocking(move || scan::tcp_scan(&host, &port_spec, timeout_ms, with_banner, max_threads))
        .await
        .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn scan_ping_one(ip: String, timeout_ms: u64) -> scan::PingResult {
    tauri::async_runtime::spawn_blocking(move || scan::ping_one(&ip, timeout_ms))
        .await
        .unwrap_or_else(|_| scan::PingResult { ip: String::new(), up: false, latency_ms: None })
}

#[tauri::command]
async fn scan_ping_sweep(
    cidr_input: String,
    timeout_ms: u64,
    max_threads: usize,
) -> Result<Vec<scan::PingResult>, String> {
    tauri::async_runtime::spawn_blocking(move || scan::ping_sweep(&cidr_input, timeout_ms, max_threads))
        .await
        .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn scan_discover_hosts(
    cidr_input: String,
    port_spec: String,
    timeout_ms: u64,
    max_threads: usize,
) -> Result<Vec<scan::HostResult>, String> {
    tauri::async_runtime::spawn_blocking(move || scan::discover_hosts(&cidr_input, &port_spec, timeout_ms, max_threads))
        .await
        .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn scan_arp_table() -> Result<Vec<scan::ArpEntry>, String> {
    tauri::async_runtime::spawn_blocking(scan::arp_table)
        .await
        .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn scan_local_network_info() -> Result<scan::LocalNetworkInfo, String> {
    tauri::async_runtime::spawn_blocking(scan::local_network_info)
        .await
        .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn netstat_snapshot() -> Result<netstat::NetstatSnapshot, String> {
    tauri::async_runtime::spawn_blocking(netstat::snapshot)
        .await
        .map_err(|e| format!("{e}"))?
}

// ---------- content & search ----------

#[tauri::command]
fn content_entity(id: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Option<content::Entity>, String> {
    store.entity_get(&id)
}

#[tauri::command]
fn content_list(
    category_prefix: String,
    kind: Option<String>,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<Vec<(String, String, String)>, String> {
    store.entity_list(&category_prefix, kind.as_deref())
}

#[tauri::command]
fn content_categories(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<String>, String> {
    store.categories()
}

#[tauri::command]
fn search_query(
    q: String,
    limit: usize,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<Vec<store::SearchHit>, String> {
    store.search(&q, limit)
}

// ---------- user data ----------

#[tauri::command]
fn app_info() -> serde_json::Value {
    serde_json::json!({
        "version": packs::APP_VERSION,
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
    })
}

#[tauri::command]
fn store_stats(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<serde_json::Value, String> {
    let (entities, packs, favorites, history) = store.stats()?;
    Ok(serde_json::json!({
        "entities": entities, "packs": packs,
        "favorites": favorites, "history": history,
    }))
}

#[tauri::command]
fn settings_get(key: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Option<String> {
    store.setting_get(&key)
}

#[tauri::command]
fn settings_set(key: String, value: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.setting_set(&key, &value)
}

#[tauri::command]
fn fav_add(id: String, title: String, kind: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.favorite_add(&id, &title, &kind)
}

#[tauri::command]
fn fav_remove(id: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.favorite_remove(&id)
}

#[tauri::command]
fn fav_list(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<(String, String, String)>, String> {
    store.favorites()
}

#[tauri::command]
fn fav_is(id: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> bool {
    store.is_favorite(&id)
}

#[tauri::command]
fn pin_add(
    id: String,
    title: String,
    kind: String,
    route: String,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<(), String> {
    store.pin_add(&id, &title, &kind, &route)
}

#[tauri::command]
fn pin_remove(id: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.pin_remove(&id)
}

#[tauri::command]
fn pin_list(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<(String, String, String, String)>, String> {
    store.pins()
}

#[tauri::command]
fn history_add(id: String, title: String, kind: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.history_add(&id, &title, &kind)
}

#[tauri::command]
fn history_list(limit: usize, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<store::HistoryEntry>, String> {
    store.history(limit)
}

#[tauri::command]
fn history_clear(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    store.history_clear()
}

// ---------- packs ----------

#[tauri::command]
fn pack_list(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<store::PackRow>, String> {
    store.packs()
}

#[tauri::command]
fn pack_available(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<Vec<packs::RegistryEntry>, String> {
    let url = store
        .setting_get("packs.registryUrl")
        .unwrap_or_else(|| DEFAULT_REGISTRY_URL.to_string());
    let proxy = store.setting_get("network.proxy");
    packs::fetch_registry(&url, proxy.as_deref()).map(|idx| idx.packs)
}

#[tauri::command]
async fn pack_install_file(path: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<packs::InstallReport, String> {
    let store = store.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let packs_root = store.data_dir()?.join("packs");
        packs::install_from_zip(Path::new(&path), &packs_root, &store)
    })
    .await
    .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn pack_install_registry(
    id: String,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<packs::InstallReport, String> {
    let store = store.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let url = store
            .setting_get("packs.registryUrl")
            .unwrap_or_else(|| DEFAULT_REGISTRY_URL.to_string());
        let proxy = store.setting_get("network.proxy");
        let index = packs::fetch_registry(&url, proxy.as_deref())?;
        let entry = index
            .packs
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| format!("pack '{id}' not in registry"))?;
        let packs_root = store.data_dir()?.join("packs");
        let tmp = store.data_dir()?;
        packs::install_from_registry(&entry, &packs_root, &tmp, proxy.as_deref(), &store)
    })
    .await
    .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn pack_remove(id: String, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    let store = store.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let packs_root = store.data_dir()?.join("packs");
        packs::remove_pack(&id, &packs_root, &store)
    })
    .await
    .map_err(|e| format!("{e}"))?
}

#[tauri::command]
async fn pack_ensure_core(store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<bool, String> {
    let store = store.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let packs_root = store.data_dir()?.join("packs");
        packs::ensure_core_installed(&store, &packs_root)
    })
    .await
    .map_err(|e| format!("{e}"))?
}

use std::path::Path;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let store = Store::open(&data_dir)
                .map_err(|e| format!("could not open local database: {e}"))?;
            let packs_root = data_dir.join("packs");
            if let Err(e) = packs::ensure_core_installed(&store, &packs_root) {
                eprintln!("core pack install failed: {e}");
            }
            app.manage(std::sync::Arc::new(store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cidr_calculate,
            cidr_split,
            cidr_range_to_cidrs,
            cidr_contains,
            port_lookup,
            port_search,
            port_describe_range,
            packet_parse,
            packet_build_tcp,
            packet_build_tcp_hex,
            packet_hexdump,
            scan_tcp,
            scan_ping_one,
            scan_ping_sweep,
            scan_discover_hosts,
            scan_arp_table,
            scan_local_network_info,
            netstat_snapshot,
            toolcmds::ipv6_calculate,
            toolcmds::ipv6_expand,
            toolcmds::ipv6_eui64,
            toolcmds::calc_transfer,
            toolcmds::calc_mtu,
            toolcmds::calc_overhead,
            toolcmds::num_convert,
            toolcmds::ipv4_forms,
            toolcmds::mac_forms,
            toolcmds::base64_convert,
            toolcmds::url_convert,
            toolcmds::hex_convert,
            toolcmds::timestamp_from_unix_cmd,
            toolcmds::timestamp_parse_cmd,
            toolcmds::uuid_gen,
            toolcmds::gen_mac,
            toolcmds::gen_ipv4,
            toolcmds::gen_password,
            toolcmds::hash_text_cmd,
            toolcmds::json_format_cmd,
            toolcmds::regex_test_cmd,
            toolcmds::diff_lines_cmd,
            toolcmds::case_convert_cmd,
            toolcmds::clean_whitespace_cmd,
            diagcmds::dns_lookup,
            diagcmds::dns_compare,
            diagcmds::http_test_cmd,
            diagcmds::sys_info_cmd,
            diagcmds::sys_processes,
            diagcmds::sys_disks,
            diagcmds::session_create,
            diagcmds::session_list,
            diagcmds::session_add_result,
            diagcmds::session_results,
            diagcmds::session_set_notes,
            diagcmds::session_delete,
            diagcmds::session_export_md,
            diagcmds::save_text_file,
            updatecmds::check_update,
            updatecmds::download_and_install_update,
            updatecmds::factory_reset,
            packetcaps::pcap_parse_file,
            packetcaps::rb_progress_get,
            packetcaps::rb_progress_set,
            packetcaps::rb_progress_clear,
            content_entity,
            content_list,
            content_categories,
            search_query,
            app_info,
            store_stats,
            settings_get,
            settings_set,
            fav_add,
            fav_remove,
            fav_list,
            fav_is,
            pin_add,
            pin_remove,
            pin_list,
            history_add,
            history_list,
            history_clear,
            pack_list,
            pack_available,
            pack_install_file,
            pack_install_registry,
            pack_remove,
            pack_ensure_core,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Network++");
}
