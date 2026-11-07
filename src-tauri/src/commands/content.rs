use serde::Serialize;
use std::path::Path;

/// The nine content templates from docs/PRODUCT.md. Every entity in every
/// pack must declare one of these kinds.
pub const KINDS: &[&str] = &[
    "command",
    "protocol",
    "vendor",
    "tool",
    "runbook",
    "error",
    "port",
    "configuration",
    "standard",
];

pub const RISKS: &[&str] = &["info", "low", "medium", "high"];

#[derive(Debug, Clone, Serialize)]
pub struct Entity {
    pub id: String,
    pub pack_id: String,
    pub kind: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub risk: Option<String>,
    pub version: Option<String>,
    pub body: String,
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 160
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '/' | '-' | '_'))
}

/// Parse one `---\n…toml…\n---\nbody` markdown entity.
pub fn parse_entity_source(text: &str, pack_id: &str) -> Result<Entity, String> {
    // Normalize CRLF (git autocrlf checkouts) and strip BOM before parsing.
    let text = text.replace("\r\n", "\n");
    let text = text.trim_start_matches('\u{feff}');
    let rest = text
        .strip_prefix("---")
        .ok_or("missing frontmatter — file must start with ---")?;
    let end = rest
        .find("\n---")
        .ok_or("unterminated frontmatter — closing --- not found")?;
    let frontmatter = &rest[..end];
    let body = rest[end + 4..].trim_start_matches('\n').trim_end().to_string();

    let table: toml::Table = toml::from_str(frontmatter)
        .map_err(|e| format!("invalid TOML frontmatter: {e}"))?;

    let get_str = |key: &str| -> Result<String, String> {
        table
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("frontmatter field '{key}' is required"))
    };

    let id = get_str("id")?;
    if !valid_id(&id) {
        return Err(format!(
            "invalid id '{id}' — lowercase letters, digits, '/', '-', '_' only"
        ));
    }
    let kind = get_str("kind")?;
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!(
            "unknown kind '{kind}' — expected one of: {}",
            KINDS.join(", ")
        ));
    }
    let title = get_str("title")?;
    let category = get_str("category")?;
    if !valid_id(&category) {
        return Err(format!("invalid category '{category}'"));
    }

    let tags = table
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let risk = match table.get("risk").and_then(|v| v.as_str()) {
        None => None,
        Some(r) if RISKS.contains(&r) => Some(r.to_string()),
        Some(r) => return Err(format!("invalid risk '{r}' — expected one of: {}", RISKS.join(", "))),
    };

    let version = table
        .get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if body.trim().is_empty() {
        return Err(format!("entity '{id}' has an empty body"));
    }

    Ok(Entity {
        id,
        pack_id: pack_id.to_string(),
        kind,
        title,
        category,
        tags,
        risk,
        version,
        body,
    })
}

pub fn parse_entity_file(path: &Path, pack_id: &str) -> Result<Entity, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse_entity_source(&text, pack_id).map_err(|e| format!("{}: {e}", path.display()))
}

/// Parse every `*.md` under `root` (recursively). The path relative to the
/// content root is reported for error messages. Skips files that fail with a
/// collected error list — a pack installs if at least one entity parses.
pub fn parse_pack_dir(root: &Path, pack_id: &str) -> Result<(Vec<Entity>, Vec<String>), String> {
    let mut entities = Vec::new();
    let mut errors = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                match parse_entity_file(&path, pack_id) {
                    Ok(e) => entities.push(e),
                    Err(e) => errors.push(e),
                }
            }
        }
    }
    entities.sort_by(|a, b| a.id.cmp(&b.id));
    Ok((entities, errors))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"---
id = "cmd/linux/ping"
kind = "command"
title = "ping — send ICMP echo"
category = "reference/os/linux/connectivity"
tags = ["linux", "icmp"]
risk = "low"
version = "core"

---
Sends ICMP echo requests and reports RTT.

```bash
ping -c 4 8.8.8.8
```
"#;

    #[test]
    fn parses_valid_entity() {
        let e = parse_entity_source(SAMPLE, "core").unwrap();
        assert_eq!(e.id, "cmd/linux/ping");
        assert_eq!(e.kind, "command");
        assert_eq!(e.title, "ping — send ICMP echo");
        assert_eq!(e.category, "reference/os/linux/connectivity");
        assert_eq!(e.tags, vec!["linux", "icmp"]);
        assert_eq!(e.risk.as_deref(), Some("low"));
        assert_eq!(e.version.as_deref(), Some("core"));
        assert!(e.body.contains("ICMP echo"));
        assert!(e.body.contains("```bash"));
    }

    #[test]
    fn rejects_missing_or_bad_fields() {
        assert!(parse_entity_source("no frontmatter here", "p").is_err());
        assert!(parse_entity_source("---\nid = \"x\"\n---\nbody", "p").is_err()); // missing kind etc
        assert!(parse_entity_source("---\nid = \"x\"\n---\nbody", "p").is_err());
        let bad_kind = SAMPLE.replace("command", "magic");
        assert!(parse_entity_source(&bad_kind, "p").is_err());
        let bad_risk = SAMPLE.replace("risk = \"low\"", "risk = \"yolo\"");
        assert!(parse_entity_source(&bad_risk, "p").is_err());
        let bad_id = SAMPLE.replace("cmd/linux/ping", "Bad Id!");
        assert!(parse_entity_source(&bad_id, "p").is_err());
        let empty_body = "---\nid = \"a/b\"\nkind = \"error\"\ntitle = \"t\"\ncategory = \"c/d\"\n---\n   \n";
        assert!(parse_entity_source(empty_body, "p").is_err());
    }

    #[test]
    fn parses_crlf_frontmatter() {
        // Git autocrlf checkouts deliver CRLF — the parser must tolerate it.
        let crlf = SAMPLE.replace('\n', "\r\n");
        let e = parse_entity_source(&crlf, "core").unwrap();
        assert_eq!(e.id, "cmd/linux/ping");
        assert_eq!(e.risk.as_deref(), Some("low"));
    }

    #[test]
    fn optional_fields_default() {
        let minimal = "---\nid = \"err/http-404\"\nkind = \"error\"\ntitle = \"404 Not Found\"\ncategory = \"reference/errors/http\"\n---\nServer cannot find the resource.\n";
        let e = parse_entity_source(minimal, "core").unwrap();
        assert!(e.tags.is_empty());
        assert!(e.risk.is_none());
        assert!(e.version.is_none());
    }
}
