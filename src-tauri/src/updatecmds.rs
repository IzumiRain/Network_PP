// 1.2.0: factory reset + in-app update check/download.
// The installer executor hard-allowlists the release URL of this repo —
// this command runs a downloaded binary, so it must never be a
// generic URL opener.

use crate::commands::packs;
use crate::commands::store::Store;
use serde::Serialize;

const RELEASES_API: &str = "https://api.github.com/repos/IzumiRain/Network_PP/releases/latest";
const RELEASE_DOWNLOAD_PREFIX: &str =
    "https://github.com/IzumiRain/Network_PP/releases/download/";

fn build_agent(proxy: Option<&str>) -> Result<ureq::Agent, String> {
    let mut builder = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(15))
        .timeout(std::time::Duration::from_secs(120));
    if let Some(p) = proxy {
        if !p.trim().is_empty() {
            let px = ureq::Proxy::new(p.trim())
                .map_err(|_| format!("invalid proxy '{p}'"))?;
            builder = builder.proxy(px);
        }
    }
    Ok(builder.build())
}

fn get_json(agent: &ureq::Agent, url: &str, token: Option<&str>) -> Result<serde_json::Value, String> {
    let mut req = agent.get(url).set("User-Agent", "Network++");
    if let Some(t) = token {
        if !t.trim().is_empty() {
            req = req.set("Authorization", &format!("Bearer {}", t.trim()));
        }
    }
    let resp = req.call().map_err(|e| match e {
        ureq::Error::Status(404, _) => "no published release found".to_string(),
        ureq::Error::Status(401 | 403, _) => {
            "GitHub rejected the request (401/403) — the repository is private, so a token is required (Settings → Updates)".into()
        }
        other => other.to_string(),
    })?;
    let text = resp.into_string().map_err(|e| format!("read failed: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("bad JSON from GitHub: {e}"))
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
    pub update_available: bool,
    pub release_url: String,
    pub installer_url: Option<String>,
    pub installer_name: Option<String>,
    pub installer_size: Option<u64>,
    pub notes: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn check_update(
    token: Option<String>,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<UpdateInfo, String> {
    let mut info = UpdateInfo {
        current: packs::APP_VERSION.to_string(),
        latest: String::new(),
        update_available: false,
        release_url: "https://github.com/IzumiRain/Network_PP/releases/latest".into(),
        installer_url: None,
        installer_name: None,
        installer_size: None,
        notes: None,
        error: None,
    };
    let token = token
        .or_else(|| store.setting_get("updates.githubToken"))
        .filter(|t| !t.trim().is_empty());
    let proxy = store.setting_get("network.proxy");

    let agent = match build_agent(proxy.as_deref()) {
        Ok(a) => a,
        Err(e) => {
            info.error = Some(e);
            return Ok(info);
        }
    };
    let json = match get_json(&agent, RELEASES_API, token.as_deref()) {
        Ok(j) => j,
        Err(e) => {
            info.error = Some(e);
            return Ok(info);
        }
    };

    let tag = json["tag_name"].as_str().unwrap_or("").to_string();
    info.latest = tag.trim_start_matches('v').to_string();
    info.release_url = json["html_url"]
        .as_str()
        .unwrap_or(info.release_url.as_str())
        .to_string();
    info.notes = json["body"].as_str().map(|s| s.chars().take(400).collect());

    if let Some(assets) = json["assets"].as_array() {
        for a in assets {
            let name = a["name"].as_str().unwrap_or("");
            let url = a["browser_download_url"].as_str().unwrap_or("");
            if name.ends_with("_x64-setup.exe") && url.starts_with(RELEASE_DOWNLOAD_PREFIX) {
                info.installer_url = Some(url.to_string());
                info.installer_name = Some(name.to_string());
                info.installer_size = a["size"].as_u64();
                break;
            }
        }
    }
    info.update_available = packs::compare_versions(&info.latest, &info.current)
        == std::cmp::Ordering::Greater;
    Ok(info)
}

#[tauri::command]
pub async fn download_and_install_update(
    installer_url: String,
    token: Option<String>,
    app: tauri::AppHandle,
    store: tauri::State<'_, std::sync::Arc<Store>>,
) -> Result<String, String> {
    // Trust boundary: this spawns the downloaded file. Only allow the
    // official release downloads of this exact repository, exe only.
    if !installer_url.starts_with(RELEASE_DOWNLOAD_PREFIX) || !installer_url.ends_with(".exe") {
        return Err("refusing to download: URL is not an official Network++ release asset".into());
    }
    let name = installer_url.rsplit('/').next().unwrap_or("setup.exe").to_string();
    let token = token
        .or_else(|| store.setting_get("updates.githubToken"))
        .filter(|t| !t.trim().is_empty());
    let proxy = store.setting_get("network.proxy");

    use tauri::Manager;
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let updates_dir = data_dir.join("updates");
    std::fs::create_dir_all(&updates_dir).map_err(|e| e.to_string())?;
    let dest = updates_dir.join(&name);

    let agent = build_agent(proxy.as_deref())?;
    let mut req = agent
        .get(&installer_url)
        .set("User-Agent", "Network++");
    // Private repo: the asset download needs the same Bearer token as the API.
    if let Some(t) = &token {
        req = req.set("Authorization", &format!("Bearer {}", t.trim()));
    }
    let resp = req
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(401 | 403, _) => {
                "download rejected (401/403) — the repository is private, so a token is required (Settings → Updates)".into()
            }
            other => other.to_string(),
        })?;
    let mut out = std::fs::File::create(&dest).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp.into_reader(), &mut out).map_err(|e| e.to_string())?;
    drop(out);

    // Hand over to the installer and close this instance.
    std::process::Command::new(&dest)
        .spawn()
        .map_err(|e| format!("could not launch installer: {e}"))?;
    app.exit(0);
    Ok(name)
}

#[tauri::command]
pub async fn factory_reset(app: tauri::AppHandle, store: tauri::State<'_, std::sync::Arc<Store>>) -> Result<(), String> {
    use tauri::Manager;
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let packs_root = data_dir.join("packs");
    store.factory_reset(&packs_root)?;
    packs::ensure_core_installed(&store, &data_dir.join("packs"))?;
    Ok(())
}
