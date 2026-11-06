// 0.3 tool-expansion commands: IPv6, performance math, converters,
// generators and text/data utilities. Thin wrappers over the logic modules.

use crate::commands::{calc, convert, generate, ipv6, texttools};

#[tauri::command]
pub fn ipv6_calculate(input: String) -> Result<ipv6::Ipv6Info, String> {
    ipv6::calculate(&input)
}

#[tauri::command]
pub fn ipv6_expand(input: String) -> Result<String, String> {
    ipv6::expand_input(&input)
}

#[tauri::command]
pub fn ipv6_eui64(mac: String) -> Result<String, String> {
    ipv6::eui64(&mac)
}

#[tauri::command]
pub fn calc_transfer(size_bytes: u64, bandwidth_mbps: f64) -> Result<calc::TransferResult, String> {
    calc::transfer_time(size_bytes, bandwidth_mbps)
}

#[tauri::command]
pub fn calc_mtu(mtu: u16) -> Result<calc::MtuResult, String> {
    calc::mtu_info(mtu)
}

#[tauri::command]
pub fn calc_overhead(payload_bytes: u64) -> Result<calc::OverheadResult, String> {
    calc::overhead(payload_bytes)
}

#[tauri::command]
pub fn num_convert(input: String, base: u32) -> Result<convert::NumberResult, String> {
    convert::convert_number(&input, base)
}

#[tauri::command]
pub fn ipv4_forms(input: String) -> Result<convert::Ipv4Forms, String> {
    convert::ipv4_forms(&input)
}

#[tauri::command]
pub fn mac_forms(input: String) -> Result<convert::MacForms, String> {
    convert::mac_forms(&input)
}

#[tauri::command]
pub fn base64_convert(text: String, decode: bool) -> Result<String, String> {
    if decode {
        let bytes = convert::base64_decode(&text)?;
        String::from_utf8(bytes).map_err(|_| "result is not valid UTF-8".into())
    } else {
        Ok(convert::base64_encode(text.as_bytes()))
    }
}

#[tauri::command]
pub fn url_convert(text: String, decode: bool) -> Result<String, String> {
    if decode {
        convert::url_decode(&text)
    } else {
        Ok(convert::url_encode(&text))
    }
}

#[tauri::command]
pub fn hex_convert(text: String, decode: bool) -> Result<String, String> {
    if decode {
        convert::hex_to_text(&text)
    } else {
        Ok(convert::text_to_hex(&text))
    }
}

#[tauri::command]
pub fn timestamp_from_unix_cmd(unix: i64) -> convert::TimestampResult {
    convert::timestamp_from_unix(unix)
}

#[tauri::command]
pub fn timestamp_parse_cmd(iso: String) -> Result<convert::TimestampResult, String> {
    convert::timestamp_parse(&iso)
}

#[tauri::command]
pub fn uuid_gen(count: usize) -> Result<generate::Generated, String> {
    generate::uuid_v4(count)
}

#[tauri::command]
pub fn gen_mac(count: usize) -> Result<generate::Generated, String> {
    generate::mac_addresses(count)
}

#[tauri::command]
pub fn gen_ipv4(count: usize, private_only: bool) -> Result<generate::Generated, String> {
    generate::ipv4_addresses(count, private_only)
}

#[tauri::command]
pub fn gen_password(length: usize, symbols: bool) -> Result<generate::Generated, String> {
    generate::password(length, symbols)
}

#[tauri::command]
pub fn hash_text_cmd(text: String) -> generate::Generated {
    generate::hash_text(&text)
}

#[tauri::command]
pub fn json_format_cmd(text: String, indent: usize) -> texttools::JsonResult {
    texttools::json_format(&text, indent)
}

#[tauri::command]
pub fn regex_test_cmd(pattern: String, text: String) -> texttools::RegexResult {
    texttools::regex_test(&pattern, &text)
}

#[tauri::command]
pub fn diff_lines_cmd(a: String, b: String) -> Vec<texttools::DiffLine> {
    texttools::diff_lines(&a, &b)
}

#[tauri::command]
pub fn case_convert_cmd(text: String, style: String) -> Result<String, String> {
    texttools::case_convert(&text, &style)
}

#[tauri::command]
pub fn clean_whitespace_cmd(text: String, collapse: bool, trim_lines: bool) -> String {
    texttools::clean_whitespace(&text, collapse, trim_lines)
}
