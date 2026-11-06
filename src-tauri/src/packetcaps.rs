// 0.5 commands: PCAP reading + runbook progress.

use crate::commands::{pcap, store::Store};

#[tauri::command]
pub fn pcap_parse_file(path: String) -> Result<pcap::PcapInfo, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path))?;
    if bytes.len() > 64 * 1024 * 1024 {
        return Err("file exceeds 64 MB — split the capture first".into());
    }
    pcap::parse_pcap(&bytes)
}

#[tauri::command]
pub fn rb_progress_get(entity_id: String, store: tauri::State<'_, Store>) -> Result<Vec<usize>, String> {
    store.rb_progress_get(&entity_id)
}

#[tauri::command]
pub fn rb_progress_set(
    entity_id: String,
    checked: Vec<usize>,
    store: tauri::State<'_, Store>,
) -> Result<(), String> {
    store.rb_progress_set(&entity_id, &checked)
}

#[tauri::command]
pub fn rb_progress_clear(entity_id: String, store: tauri::State<'_, Store>) -> Result<(), String> {
    store.rb_progress_clear(&entity_id)
}
