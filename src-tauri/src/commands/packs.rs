use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

use super::content;
use super::store::Store;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CORE_PACK_ID: &str = "core";
const MAX_ZIP_BYTES: u64 = 100 * 1024 * 1024;
const MAX_FILE_BYTES: u64 = 10 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    pub id: String,
    pub title: String,
    pub version: String,
    pub min_app_version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub id: String,
    pub title: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub sha256: String,
    pub url: String,
    #[serde(default)]
    pub min_app_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryIndex {
    #[serde(default)]
    pub source_url: String,
    pub packs: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallReport {
    pub id: String,
    pub title: String,
    pub version: String,
    pub entity_count: usize,
    pub warnings: Vec<String>,
}

// ---------- version handling ----------

/// Three-part dotted version compare; missing parts count as 0.
pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u64> {
        s.split('.')
            .map(|p| p.trim().parse::<u64>().unwrap_or(0))
            .collect()
    };
    let (a, b) = (parse(a), parse(b));
    for i in 0..3 {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        match av.cmp(&bv) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

fn check_app_version(min_app: &str) -> Result<(), String> {
    if compare_versions(APP_VERSION, min_app) == std::cmp::Ordering::Less {
        return Err(format!(
            "Pack requires app {min_app} or newer — running {APP_VERSION}"
        ));
    }
    Ok(())
}

pub fn valid_pack_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

// ---------- manifest ----------

pub fn parse_manifest(json: &str) -> Result<Manifest, String> {
    let m: Manifest =
        serde_json::from_str(json).map_err(|e| format!("invalid manifest.json: {e}"))?;
    if m.schema != 1 {
        return Err(format!("unsupported pack schema {} — expected 1", m.schema));
    }
    if !valid_pack_id(&m.id) {
        return Err(format!("invalid pack id '{}'", m.id));
    }
    if m.title.trim().is_empty() || m.version.trim().is_empty() {
        return Err("manifest needs title and version".into());
    }
    check_app_version(&m.min_app_version)?;
    Ok(m)
}

fn read_manifest_from_map(files: &[(&str, &[u8])]) -> Result<Manifest, String> {
    let (_, bytes) = files
        .iter()
        .find(|(name, _)| *name == "manifest.json")
        .ok_or("pack is missing manifest.json")?;
    let text = String::from_utf8_lossy(bytes);
    parse_manifest(&text)
}

// ---------- install from a plain file map (embedded core pack) ----------

pub fn install_from_files(
    files: &[(&str, &[u8])],
    packs_root: &Path,
    store: &Store,
    source: &str,
) -> Result<InstallReport, String> {
    let manifest = read_manifest_from_map(files)?;
    let target = packs_root.join(&manifest.id);
    std::fs::remove_dir_all(&target).ok();
    for (name, bytes) in files {
        if *name == "manifest.json" {
            continue;
        }
        if !name.starts_with("content/") || !name.ends_with(".md") {
            continue; // unknown extras are ignored deliberately
        }
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err(format!("{name} exceeds {MAX_FILE_BYTES} byte limit"));
        }
        let dest = target.join(name.replace('/', "\\").replace('\\', std::path::MAIN_SEPARATOR_STR));
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&dest, bytes).map_err(|e| format!("{name}: {e}"))?;
    }
    finish_install(&target, &manifest, store, source)
}

fn finish_install(
    pack_dir: &Path,
    manifest: &Manifest,
    store: &Store,
    source: &str,
) -> Result<InstallReport, String> {
    let (entities, errors) = content::parse_pack_dir(&pack_dir.join("content"), &manifest.id)?;
    if entities.is_empty() {
        return Err(format!(
            "pack '{}' contains no valid entities{}",
            manifest.id,
            if errors.is_empty() {
                String::new()
            } else {
                format!(" — first error: {}", errors[0])
            }
        ));
    }
    store.entities_replace_pack(&manifest.id, &entities)?;
    store.pack_upsert(
        &manifest.id,
        &manifest.title,
        &manifest.version,
        entities.len() as i64,
        source,
    )?;
    Ok(InstallReport {
        id: manifest.id.clone(),
        title: manifest.title.clone(),
        version: manifest.version.clone(),
        entity_count: entities.len(),
        warnings: errors.into_iter().take(10).collect(),
    })
}

// ---------- install from a zip ----------

fn safe_extract_name(name: &str) -> Option<PathBuf> {
    if name.contains("..") || name.starts_with('/') || name.starts_with('\\') {
        return None;
    }
    let rel: PathBuf = name
        .split(['/', '\\'])
        .filter(|p| !p.is_empty() && *p != ".")
        .collect();
    Some(rel)
}

pub fn install_from_zip(zip_path: &Path, packs_root: &Path, store: &Store) -> Result<InstallReport, String> {
    let meta = std::fs::metadata(zip_path).map_err(|e| format!("{}: {e}", zip_path.display()))?;
    if meta.len() > MAX_ZIP_BYTES {
        return Err(format!(
            "pack file is {} MB — limit is {} MB",
            meta.len() / (1024 * 1024),
            MAX_ZIP_BYTES / (1024 * 1024)
        ));
    }
    let file = std::fs::File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("not a valid zip: {e}"))?;

    // Manifest first.
    let mut manifest_json = String::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.name() == "manifest.json" {
            entry
                .read_to_string(&mut manifest_json)
                .map_err(|e| e.to_string())?;
            break;
        }
    }
    if manifest_json.is_empty() {
        return Err("zip is missing manifest.json at its root".into());
    }
    let manifest = parse_manifest(&manifest_json)?;

    // Extract content/*.md
    let target = packs_root.join(&manifest.id);
    std::fs::remove_dir_all(&target).ok();
    std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();
        if !name.starts_with("content/") || !name.ends_with(".md") || entry.is_dir() {
            continue;
        }
        if entry.size() > MAX_FILE_BYTES {
            return Err(format!("{name} exceeds {MAX_FILE_BYTES} byte limit"));
        }
        let rel = safe_extract_name(&name).ok_or_else(|| format!("unsafe zip entry: {name}"))?;
        let dest = target.join(rel);
        std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
        let mut buf = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        std::fs::write(&dest, &buf).map_err(|e| format!("{name}: {e}"))?;
    }

    finish_install(&target, &manifest, store, &format!("file:{}", zip_path.display()))
}

// ---------- remove / export ----------

pub fn remove_pack(pack_id: &str, packs_root: &Path, store: &Store) -> Result<(), String> {
    if pack_id == CORE_PACK_ID {
        return Err("The core pack cannot be removed — it ships with the app".into());
    }
    store.pack_remove(pack_id)?;
    std::fs::remove_dir_all(packs_root.join(pack_id)).ok();
    Ok(())
}

pub fn pack_dir(packs_root: &Path, pack_id: &str) -> PathBuf {
    packs_root.join(pack_id)
}

// ---------- registry ----------

pub fn parse_registry_json(text: &str, source_url: &str) -> Result<RegistryIndex, String> {
    let mut idx: RegistryIndex = serde_json::from_str(text)
        .map_err(|e| format!("invalid registry index: {e}"))?;
    idx.source_url = source_url.to_string();
    for p in &idx.packs {
        if !valid_pack_id(&p.id) {
            return Err(format!("registry lists invalid pack id '{}'", p.id));
        }
        if p.url.is_empty() {
            return Err(format!("pack '{}' has no download url", p.id));
        }
        if !p.url.starts_with("https://") {
            return Err(format!("pack '{}' url must be https", p.id));
        }
    }
    Ok(idx)
}

fn http_agent(proxy: Option<&str>) -> Result<ureq::Agent, String> {
    let mut builder = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(60));
    if let Some(p) = proxy {
        if !p.trim().is_empty() {
            let proxy = ureq::Proxy::new(p.trim())
                .map_err(|_| format!("invalid proxy '{p}' — use socks5://host:port or http://host:port"))?;
            builder = builder.proxy(proxy);
        }
    }
    Ok(builder.build())
}

pub fn fetch_registry(registry_url: &str, proxy: Option<&str>) -> Result<RegistryIndex, String> {
    let agent = http_agent(proxy)?;
    let resp = agent
        .get(registry_url)
        .call()
        .map_err(|e| format!("registry fetch failed: {e}"))?;
    let text = resp.into_string().map_err(|e| e.to_string())?;
    parse_registry_json(&text, registry_url)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

pub fn verify_sha256(file: &Path, expected: &str) -> Result<(), String> {
    let expected = expected.trim().to_lowercase();
    if expected.is_empty() {
        return Ok(()); // no checksum published — allowed but registry should provide one
    }
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let actual = sha256_hex(&bytes);
    if actual != expected {
        return Err(format!(
            "checksum mismatch — expected {expected}, got {actual}"
        ));
    }
    Ok(())
}

pub fn download_to(file_url: &str, dest: &Path, proxy: Option<&str>) -> Result<(), String> {
    let agent = http_agent(proxy)?;
    let resp = agent
        .get(file_url)
        .call()
        .map_err(|e| format!("download failed: {e}"))?;
    let mut out = std::fs::File::create(dest).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp.into_reader(), &mut out).map_err(|e| e.to_string())?;
    Ok(())
}

/// Download a registry entry and install it. Returns the install report.
pub fn install_from_registry(
    entry: &RegistryEntry,
    packs_root: &Path,
    tmp_dir: &Path,
    proxy: Option<&str>,
    store: &Store,
) -> Result<InstallReport, String> {
    check_app_version(&entry.min_app_version)?;
    let dest = tmp_dir.join(format!("pack-{}-{}.zip", entry.id, entry.version));
    download_to(&entry.url, &dest, proxy)?;
    let result = (|| {
        verify_sha256(&dest, &entry.sha256)?;
        install_from_zip(&dest, packs_root, store)
    })();
    std::fs::remove_file(&dest).ok();
    result
}

// ---------- embedded core pack ----------

pub static CORE_PACK: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/../packs-src/core");

fn core_files() -> Vec<(&'static str, &'static [u8])> {
    fn walk(
        dir: &'static include_dir::Dir<'static>,
        out: &mut Vec<(&'static str, &'static [u8])>,
    ) {
        for f in dir.files() {
            out.push((f.path().to_str().expect("utf8 path"), f.contents()));
        }
        for d in dir.dirs() {
            walk(d, out);
        }
    }
    let mut out = Vec::new();
    walk(&CORE_PACK, &mut out);
    out
}

/// Install the bundled core pack on first run (or after an app update that
/// bumps its version). Returns true when a (re)install happened.
pub fn ensure_core_installed(store: &Store, packs_root: &Path) -> Result<bool, String> {
    let files = core_files();
    let manifest = read_manifest_from_map(&files)?;
    let needs_install = match store
        .packs()?
        .into_iter()
        .find(|p| p.id == CORE_PACK_ID)
    {
        Some(existing) => compare_versions(&manifest.version, &existing.version) == std::cmp::Ordering::Greater,
        None => true,
    };
    if !needs_install {
        return Ok(false);
    }
    install_from_files(&files, packs_root, store, "bundled")?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::store::Store;

    fn temp_root(tag: &str) -> (PathBuf, Store) {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "netpp-packs-{}-{}-{n}",
            tag,
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let store = Store::open(&dir.join("db")).unwrap();
        (dir, store)
    }

    const MANIFEST: &str = r#"{
        "schema": 1,
        "id": "testpack",
        "title": "Test Pack",
        "version": "1.2.0",
        "min_app_version": "0.2.0",
        "description": "test"
    }"#;

    const ENTITY_A: &str = "---\nid = \"cmd/test/one\"\nkind = \"command\"\ntitle = \"One\"\ncategory = \"reference/testing\"\n---\nBody one about ping.\n";
    const ENTITY_B: &str = "---\nid = \"cmd/test/two\"\nkind = \"command\"\ntitle = \"Two\"\ncategory = \"reference/testing\"\n---\nBody two about dns.\n";

    fn build_zip(path: &Path, entries: Vec<(&str, &[u8])>) {
        let file = std::fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default();
        for (name, bytes) in entries {
            zip.start_file(name, opts).unwrap();
            std::io::Write::write_all(&mut zip, bytes).unwrap();
        }
        zip.finish().unwrap();
    }

    #[test]
    fn version_compare() {
        use std::cmp::Ordering::*;
        assert_eq!(compare_versions("1.2.0", "1.2.0"), Equal);
        assert_eq!(compare_versions("1.10.0", "1.9.0"), Greater);
        assert_eq!(compare_versions("0.2", "0.2.0"), Equal);
        assert_eq!(compare_versions("1.0", "1.0.1"), Less);
    }

    #[test]
    fn manifest_validation() {
        assert!(parse_manifest(MANIFEST).is_ok());
        assert!(parse_manifest(&MANIFEST.replace("\"schema\": 1", "\"schema\": 2")).is_err());
        assert!(parse_manifest(&MANIFEST.replace("testpack", "Bad_ID")).is_err());
        assert!(parse_manifest(&MANIFEST.replace("\"min_app_version\": \"0.2.0\"", "\"min_app_version\": \"99.0.0\"")).is_err());
    }

    #[test]
    fn install_zip_end_to_end() {
        let (root, store) = temp_root("install");
        let zip_path = root.join("testpack.zip");
        build_zip(
            &zip_path,
            vec![
                ("manifest.json", MANIFEST.as_bytes()),
                ("content/one.md", ENTITY_A.as_bytes()),
                ("content/sub/two.md", ENTITY_B.as_bytes()),
            ],
        );
        let packs_root = root.join("packs");
        let report = install_from_zip(&zip_path, &packs_root, &store).unwrap();
        assert_eq!(report.id, "testpack");
        assert_eq!(report.entity_count, 2);
        assert!(store.entity_get("cmd/test/one").unwrap().is_some());
        assert_eq!(store.stats().unwrap().0, 2);

        // reinstall replaces, not duplicates
        install_from_zip(&zip_path, &packs_root, &store).unwrap();
        assert_eq!(store.stats().unwrap().0, 2);

        remove_pack("testpack", &packs_root, &store).unwrap();
        assert_eq!(store.stats().unwrap().0, 0);
    }

    #[test]
    fn rejects_zip_slip_and_missing_manifest() {
        let (root, store) = temp_root("slip");
        let packs_root = root.join("packs");
        let zip_path = root.join("evil.zip");
        build_zip(
            &zip_path,
            vec![
                ("manifest.json", MANIFEST.as_bytes()),
                ("content/../../evil.md", ENTITY_A.as_bytes()),
            ],
        );
        // The unsafe entry is skipped, no valid content remains → install fails,
        // nothing is registered, and nothing escapes packs_root.
        assert!(install_from_zip(&zip_path, &packs_root, &store).is_err());
        assert_eq!(store.stats().unwrap().0, 0);
        assert!(!root.join("evil.md").exists());
        assert!(!packs_root.parent().unwrap().join("evil.md").exists());

        let nomani = root.join("nomani.zip");
        build_zip(&nomani, vec![("content/a.md", ENTITY_A.as_bytes())]);
        assert!(install_from_zip(&nomani, &packs_root, &store).is_err());
    }

    #[test]
    fn core_pack_embeds_and_installs() {
        let (root, store) = temp_root("core");
        let packs_root = root.join("packs");
        let installed = ensure_core_installed(&store, &packs_root).unwrap();
        assert!(installed, "core pack must install on first run");
        let (entities, packs, _, _) = store.stats().unwrap();
        assert!(entities > 10, "core pack should ship real content, got {entities}");
        assert_eq!(packs, 1);
        // second call is a no-op
        assert!(!ensure_core_installed(&store, &packs_root).unwrap());
    }

    #[test]
    fn registry_json_validation() {
        let ok = r#"{"schema":1,"packs":[{"id":"vendor-cisco","title":"Cisco","version":"1.0.0","url":"https://cdn.jsdelivr.net/gh/x/y/p.zip","sha256":"abc"}]}"#;
        let idx = parse_registry_json(ok, "https://x/index.json").unwrap();
        assert_eq!(idx.packs.len(), 1);
        assert!(parse_registry_json(ok.replace("https://", "http://").as_str(), "u").is_err());
        assert!(parse_registry_json("{\"schema\":1,\"packs\":[{\"id\":\"Bad\",\"url\":\"https://x\"}]}", "u").is_err());
    }

    #[test]
    fn sha256_known_vector() {
        assert_eq!(
            sha256_hex(b"hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}
