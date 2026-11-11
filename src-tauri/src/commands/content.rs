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
    /// v2 document model (docs/PACK-SPEC.md): `##` sections of the body,
    /// each with the tags from its `<!-- section-tags: … -->` marker.
    /// Derived from `body` at parse/load time; never stored separately.
    #[serde(default)]
    pub sections: Vec<EntitySection>,
}

// ---------------------------------------------------------------------------
// v2: generic category-path parser
// ---------------------------------------------------------------------------
//
// Category slugs are the single source of structure — the frontend tree and
// any future UI must derive everything from the path, never from a hardcoded
// list of vendors, products or sections. Only the two schema roots
// (`reference`, `runbooks`) and the `vendors` marker are structural keywords;
// every other segment is data.
//
// Grammar (roles are assigned positionally, by shape — never by name):
//
//   reference/vendors/<company>/<product>/<model>[/<topic>…]
//       root / category / company / device / model / topic…
//   reference/<area>[/<group>[/<topic>…]]
//       root / category / group / topic…
//   runbooks/<area>[/<topic>…]
//       root / area / topic…
//
// Tree mapping (mobile redesign): Category → Company → Device → Model/id
// are the first four roles of the vendors shape; other shapes expose their
// own roles and UIs render them generically in path order.

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CategorySegment {
    /// Raw slug segment, e.g. `fortinet`.
    pub slug: String,
    /// Positional role, e.g. `category`, `company`, `device`, `model`.
    pub role: String,
    /// Cumulative path up to and including this segment.
    pub path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CategoryPath {
    /// The original category slug, unchanged.
    pub category: String,
    /// First segment (`reference`, `runbooks`, …).
    pub root: String,
    pub segments: Vec<CategorySegment>,
}

impl CategoryPath {
    pub fn segment_by_role(&self, role: &str) -> Option<&CategorySegment> {
        self.segments.iter().find(|s| s.role == role)
    }

    /// The model segment of a vendor document path, if the path has one.
    pub fn model(&self) -> Option<&CategorySegment> {
        self.segment_by_role("model")
    }

    /// Slug segments in path order (display names are derived from these
    /// generically by prettifying the slug — no lookup tables).
    pub fn slugs(&self) -> Vec<&str> {
        self.segments.iter().map(|s| s.slug.as_str()).collect()
    }
}

pub fn parse_category_path(category: &str) -> CategoryPath {
    let parts: Vec<&str> = category.split('/').filter(|p| !p.is_empty()).collect();
    let root = parts.first().copied().unwrap_or("").to_string();

    let roles: Vec<&str> = if parts.len() > 1 && parts[0] == "reference" && parts[1] == "vendors" {
        // root, category, company, device, model, topic…
        let mut roles = vec!["root", "category", "company", "device", "model"];
        while roles.len() < parts.len() {
            roles.push("topic");
        }
        roles.truncate(parts.len());
        roles
    } else if !parts.is_empty() && parts[0] == "runbooks" {
        let mut roles = vec!["root", "area"];
        while roles.len() < parts.len() {
            roles.push("topic");
        }
        roles.truncate(parts.len());
        roles
    } else if !parts.is_empty() {
        // Generic reference/<area>/… (and any future root): category, group, topic…
        let mut roles = vec!["root", "category", "group"];
        while roles.len() < parts.len() {
            roles.push("topic");
        }
        roles.truncate(parts.len());
        roles
    } else {
        Vec::new()
    };

    let mut path = String::new();
    let segments = parts
        .iter()
        .enumerate()
        .map(|(i, slug)| {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(slug);
            CategorySegment {
                slug: slug.to_string(),
                role: roles.get(i).copied().unwrap_or("topic").to_string(),
                path: path.clone(),
            }
        })
        .collect();

    CategoryPath {
        category: category.to_string(),
        root,
        segments,
    }
}

// ---------------------------------------------------------------------------
// v2: section extraction for document entities
// ---------------------------------------------------------------------------
//
// One .md per Model/id; inside the body, every `##` heading starts a section
// and a marker directly under the heading carries that section's own tags:
//
//   ## LTM — pools and monitors
//   <!-- section-tags: ltm, pools, monitors -->
//
// The marker is an HTML comment: invisible in rendered Markdown, trivial to
// parse, and stripped from the section body returned here.

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EntitySection {
    /// Heading text without the leading `##`. Empty for the preamble
    /// (content before the first `##` heading), when one exists.
    pub title: String,
    pub tags: Vec<String>,
    /// Section content with the section-tags marker removed.
    pub body: String,
}

fn parse_section_tags_marker(line: &str) -> Option<Vec<String>> {
    let inner = line
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    let list = inner.strip_prefix("section-tags:")?;
    Some(
        list.split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect(),
    )
}

pub fn extract_sections(body: &str) -> Vec<EntitySection> {
    let mut sections: Vec<EntitySection> = Vec::new();
    let mut title = String::new();
    let mut tags: Vec<String> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    // True while the next marker line still belongs to the current heading
    // (marker must sit directly under the heading; blank lines tolerated).
    let mut marker_window = false;

    let flush = |title: &str, tags: &[String], lines: &[String], out: &mut Vec<EntitySection>| {
        let text = lines.join("\n").trim().to_string();
        if !text.is_empty() || !title.is_empty() {
            out.push(EntitySection {
                title: title.to_string(),
                tags: tags.to_vec(),
                body: text,
            });
        }
    };

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ").or_else(|| line.strip_prefix("##\t")) {
            // `###` and deeper are content, not sections: strip_prefix("## ")
            // cannot match "### x" (third char is '#', not ' ').
            flush(&title, &tags, &lines, &mut sections);
            title = heading.trim().to_string();
            tags = Vec::new();
            lines = Vec::new();
            marker_window = true;
            continue;
        }
        if marker_window {
            if line.trim().is_empty() {
                continue; // blank line between heading and marker: skip, keep window open
            }
            if let Some(found) = parse_section_tags_marker(line) {
                tags = found;
                marker_window = false;
                continue; // marker is metadata, not body content
            }
            marker_window = false;
        }
        lines.push(line.to_string());
    }
    flush(&title, &tags, &lines, &mut sections);
    sections
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

    let sections = extract_sections(&body);
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
        sections,
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

    // ---- v2: generic category-path parser ----

    fn roles(p: &CategoryPath) -> Vec<&str> {
        p.segments.iter().map(|s| s.role.as_str()).collect()
    }

    #[test]
    fn category_path_vendors_is_company_device_model() {
        let p = parse_category_path("reference/vendors/f5/bigip/4000");
        assert_eq!(p.root, "reference");
        assert_eq!(p.slugs(), vec!["reference", "vendors", "f5", "bigip", "4000"]);
        assert_eq!(roles(&p), vec!["root", "category", "company", "device", "model"]);
        assert_eq!(p.segment_by_role("company").unwrap().slug, "f5");
        assert_eq!(p.segment_by_role("device").unwrap().slug, "bigip");
        assert_eq!(p.model().unwrap().slug, "4000");
        assert_eq!(p.model().unwrap().path, "reference/vendors/f5/bigip/4000");

        // Any company/product works — nothing is name-matched.
        let p2 = parse_category_path("reference/vendors/some-new-co/widget/x1/protocols");
        assert_eq!(
            roles(&p2),
            vec!["root", "category", "company", "device", "model", "topic"]
        );
        assert_eq!(p2.model().unwrap().slug, "x1");
    }

    #[test]
    fn category_path_generic_reference_shapes() {
        let os = parse_category_path("reference/os/linux/ssh");
        assert_eq!(roles(&os), vec!["root", "category", "group", "topic"]);
        assert!(os.model().is_none());

        let net = parse_category_path("reference/networking/addressing");
        assert_eq!(roles(&net), vec!["root", "category", "group"]);

        let mon = parse_category_path("reference/monitoring/splunk/eval-computed-fields");
        assert_eq!(roles(&mon), vec!["root", "category", "group", "topic"]);

        let err = parse_category_path("reference/errors/http");
        assert_eq!(roles(&err), vec!["root", "category", "group"]);
        assert_eq!(err.segment_by_role("category").unwrap().slug, "errors");
    }

    #[test]
    fn category_path_runbooks_and_edges() {
        let rb = parse_category_path("runbooks/fortigate");
        assert_eq!(rb.root, "runbooks");
        assert_eq!(roles(&rb), vec!["root", "area"]);

        let rb2 = parse_category_path("runbooks/network/dns");
        assert_eq!(roles(&rb2), vec!["root", "area", "topic"]);

        let empty = parse_category_path("");
        assert!(empty.segments.is_empty());
        assert_eq!(empty.root, "");

        let one = parse_category_path("reference");
        assert_eq!(roles(&one), vec!["root"]);

        // stray slashes are ignored
        let sloppy = parse_category_path("/reference//vendors/f5/bigip/4000/");
        assert_eq!(sloppy.slugs(), vec!["reference", "vendors", "f5", "bigip", "4000"]);
    }

    // ---- v2: section extraction ----

    #[test]
    fn extract_sections_reads_tags_markers() {
        let body = "Intro text.\n\n## System\n<!-- section-tags: system, tmsh -->\nVersion and hardware.\n\n## LTM — pools\n<!-- section-tags: ltm, pools -->\nPool body.\n### A subsection\nStill pool body.\n\n## HA\nNo marker here.\n";
        let s = extract_sections(body);
        assert_eq!(s.len(), 4);
        assert_eq!(s[0].title, ""); // preamble
        assert!(s[0].body.contains("Intro text."));
        assert_eq!(s[1].title, "System");
        assert_eq!(s[1].tags, vec!["system", "tmsh"]);
        assert!(!s[1].body.contains("section-tags")); // marker stripped
        assert!(s[1].body.contains("Version and hardware."));
        assert_eq!(s[2].title, "LTM — pools");
        assert_eq!(s[2].tags, vec!["ltm", "pools"]);
        assert!(s[2].body.contains("### A subsection")); // ### stays content
        assert_eq!(s[3].title, "HA");
        assert!(s[3].tags.is_empty());
    }

    #[test]
    fn marker_after_content_is_not_a_tag() {
        // A marker that is NOT directly under a heading is ordinary content.
        let body = "## Notes\nSome text first.\n<!-- section-tags: late -->\n";
        let s = extract_sections(body);
        assert_eq!(s.len(), 1);
        assert!(s[0].tags.is_empty());
        assert!(s[0].body.contains("section-tags"));
    }

    #[test]
    fn entity_parse_populates_sections() {
        let doc = "---\nid = \"doc/f5/bigip/4000\"\nkind = \"vendor\"\ntitle = \"F5 BIG-IP 4000\"\ncategory = \"reference/vendors/f5/bigip/4000\"\n---\nOverview.\n\n## System\n<!-- section-tags: system -->\nBody.\n";
        let e = parse_entity_source(doc, "vendor-f5-bigip").unwrap();
        assert_eq!(e.sections.len(), 2);
        assert_eq!(e.sections[1].title, "System");
        assert_eq!(e.sections[1].tags, vec!["system"]);
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
