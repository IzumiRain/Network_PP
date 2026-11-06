use rusqlite::Connection;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct Store {
    pub conn: Mutex<Connection>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub category: String,
    pub pack: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub visited_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackRow {
    pub id: String,
    pub title: String,
    pub version: String,
    pub installed_at: i64,
    pub entity_count: i64,
    pub source: String,
}

fn open_db(path: &Path) -> Result<Connection, String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS packs (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            version TEXT NOT NULL,
            installed_at INTEGER NOT NULL,
            entity_count INTEGER NOT NULL,
            source TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS favorites (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            kind TEXT NOT NULL,
            added_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS pins (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            kind TEXT NOT NULL,
            route TEXT NOT NULL,
            added_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS history (
            id TEXT NOT NULL,
            title TEXT NOT NULL,
            kind TEXT NOT NULL,
            visited_at INTEGER NOT NULL,
            PRIMARY KEY (id, visited_at)
        );

        CREATE TABLE IF NOT EXISTS entities (
            id TEXT PRIMARY KEY,
            pack_id TEXT NOT NULL,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            category TEXT NOT NULL,
            tags TEXT NOT NULL DEFAULT '[]',
            risk TEXT,
            version TEXT,
            body TEXT NOT NULL,
            updated_at INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_entities_kind ON entities(kind);
        CREATE INDEX IF NOT EXISTS idx_entities_category ON entities(category);
        CREATE INDEX IF NOT EXISTS idx_entities_pack ON entities(pack_id);

        CREATE VIRTUAL TABLE IF NOT EXISTS entities_fts USING fts5(
            id UNINDEXED, title, body, category, tokenize = 'unicode61'
        );
        "#,
    )
    .map_err(|e| e.to_string())?;
    Ok(conn)
}

impl Store {
    pub fn open(app_data_dir: &Path) -> Result<Self, String> {
        let db_path = app_data_dir.join("networkpp.db");
        let conn = open_db(&db_path)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn data_dir(&self) -> Result<PathBuf, String> {
        // Directory of the database file — packs live next to it.
        let conn = self.conn.lock().unwrap();
        let path = conn
            .query_row("PRAGMA database_list", [], |row| row.get::<_, String>(2))
            .map_err(|e| e.to_string())?;
        PathBuf::from(path)
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| "no parent dir".into())
    }

    // ---------- settings ----------

    pub fn setting_get(&self, key: &str) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        )
        .ok()
    }

    pub fn setting_set(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            [key, value],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    // ---------- favorites ----------

    pub fn favorite_add(&self, id: &str, title: &str, kind: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO favorites (id, title, kind, added_at) VALUES (?1, ?2, ?3, strftime('%s','now'))",
            [id, title, kind],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn favorite_remove(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM favorites WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn favorites(&self) -> Result<Vec<(String, String, String)>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, title, kind FROM favorites ORDER BY added_at DESC")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT 1 FROM favorites WHERE id = ?1",
            [id],
            |_| Ok(()),
        )
        .is_ok()
    }

    // ---------- pins ----------

    pub fn pin_add(&self, id: &str, title: &str, kind: &str, route: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO pins (id, title, kind, route, added_at) VALUES (?1, ?2, ?3, ?4, strftime('%s','now'))",
            [id, title, kind, route],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pin_remove(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM pins WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pins(&self) -> Result<Vec<(String, String, String, String)>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, title, kind, route FROM pins ORDER BY added_at DESC")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    // ---------- history ----------

    pub fn history_add(&self, id: &str, title: &str, kind: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO history (id, title, kind, visited_at) VALUES (?1, ?2, ?3, strftime('%s','now'))",
            [id, title, kind],
        )
        .map_err(|e| e.to_string())?;
        // keep the table bounded
        conn.execute(
            "DELETE FROM history WHERE visited_at NOT IN
             (SELECT DISTINCT visited_at FROM history ORDER BY visited_at DESC LIMIT 500)",
            [],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn history(&self, limit: usize) -> Result<Vec<HistoryEntry>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, title, kind, MAX(visited_at) FROM history
                 GROUP BY id ORDER BY MAX(visited_at) DESC LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([limit as i64], |r| {
                Ok(HistoryEntry {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    kind: r.get(2)?,
                    visited_at: r.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn history_clear(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // ---------- entities (pack content) ----------

    pub fn entities_replace_pack(
        &self,
        pack_id: &str,
        entities: &[super::content::Entity],
    ) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM entities WHERE pack_id = ?1", [pack_id])
            .map_err(|e| e.to_string())?;
        tx.execute(
            "DELETE FROM entities_fts WHERE id IN (SELECT id FROM entities WHERE pack_id = ?1)",
            [pack_id],
        )
        .map_err(|e| e.to_string())?;
        for e in entities {
            let tags = serde_json::to_string(&e.tags).unwrap_or_else(|_| "[]".into());
            tx.execute(
                "INSERT OR REPLACE INTO entities
                 (id, pack_id, kind, title, category, tags, risk, version, body, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, strftime('%s','now'))",
                rusqlite::params![
                    e.id,
                    pack_id,
                    e.kind,
                    e.title,
                    e.category,
                    tags,
                    e.risk,
                    e.version,
                    e.body,
                ],
            )
            .map_err(|err| format!("entity {}: {err}", e.id))?;
            tx.execute(
                "INSERT INTO entities_fts (id, title, body, category) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![e.id, e.title, e.body, e.category],
            )
            .map_err(|err| format!("fts {}: {err}", e.id))?;
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn entity_get(&self, id: &str) -> Result<Option<super::content::Entity>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, pack_id, kind, title, category, tags, risk, version, body
                 FROM entities WHERE id = ?1",
            )
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map([id], |r| {
                Ok(super::content::Entity {
                    id: r.get(0)?,
                    pack_id: r.get(1)?,
                    kind: r.get(2)?,
                    title: r.get(3)?,
                    category: r.get(4)?,
                    tags: serde_json::from_str(&r.get::<_, String>(5).unwrap_or_default())
                        .unwrap_or_default(),
                    risk: r.get(6)?,
                    version: r.get(7)?,
                    body: r.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?;
        match rows.next() {
            Some(Ok(e)) => Ok(Some(e)),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    }

    pub fn entity_list(&self, category_prefix: &str, kind: Option<&str>) -> Result<Vec<(String, String, String)>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, title, kind FROM entities
                 WHERE category LIKE ?1 || '%' AND (?2 IS NULL OR kind = ?2)
                 ORDER BY title",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(
                rusqlite::params![format!("{category_prefix}%"), kind],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn categories(&self) -> Result<Vec<String>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT DISTINCT category FROM entities ORDER BY category")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| r.get(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, String> {
        let conn = self.conn.lock().unwrap();
        // FTS5 query: quote terms to avoid syntax errors from user input.
        let q: String = query
            .split_whitespace()
            .map(|t| format!("\"{}\"", t.replace('"', "")))
            .collect::<Vec<_>>()
            .join(" ");
        if q.is_empty() {
            return Ok(Vec::new());
        }
        let mut stmt = conn
            .prepare(
                "SELECT f.id, e.kind, e.title, e.category, e.pack_id,
                        snippet(entities_fts, 2, '[', ']', '…', 12)
                 FROM entities_fts f
                 JOIN entities e ON e.id = f.id
                 WHERE entities_fts MATCH ?1
                 ORDER BY rank LIMIT ?2",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(rusqlite::params![q, limit as i64], |r| {
                Ok(SearchHit {
                    id: r.get(0)?,
                    kind: r.get(1)?,
                    title: r.get(2)?,
                    category: r.get(3)?,
                    pack: r.get(4)?,
                    snippet: r.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    // ---------- packs ----------

    pub fn pack_upsert(
        &self,
        id: &str,
        title: &str,
        version: &str,
        entity_count: i64,
        source: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO packs (id, title, version, installed_at, entity_count, source)
             VALUES (?1, ?2, ?3, strftime('%s','now'), ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET
               title = ?2, version = ?3, installed_at = strftime('%s','now'),
               entity_count = ?4, source = ?5",
            rusqlite::params![id, title, version, entity_count, source],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pack_remove(&self, id: &str) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute(
            "DELETE FROM entities_fts WHERE id IN (SELECT id FROM entities WHERE pack_id = ?1)",
            [id],
        )
        .map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM entities WHERE pack_id = ?1", [id])
            .map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM packs WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn packs(&self) -> Result<Vec<PackRow>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, title, version, installed_at, entity_count, source
                 FROM packs ORDER BY installed_at DESC",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| {
                Ok(PackRow {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    version: r.get(2)?,
                    installed_at: r.get(3)?,
                    entity_count: r.get(4)?,
                    source: r.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn stats(&self) -> Result<(i64, i64, i64, i64), String> {
        let conn = self.conn.lock().unwrap();
        let count = |sql: &str| -> Result<i64, String> {
            conn.query_row(sql, [], |r| r.get(0)).map_err(|e| e.to_string())
        };
        let entities = count("SELECT COUNT(*) FROM entities")?;
        let packs = count("SELECT COUNT(*) FROM packs")?;
        let favorites = count("SELECT COUNT(*) FROM favorites")?;
        let history = count("SELECT COUNT(*) FROM history")?;
        Ok((entities, packs, favorites, history))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store(tag: &str) -> (Store, tempdir::TempDirGuard) {
        let dir = tempdir::create(tag);
        let store = Store::open(&dir.path).expect("open store");
        (store, dir)
    }

    // Minimal scoped temp dirs so tests never write outside the workspace.
    mod tempdir {
        use std::path::PathBuf;
        use std::sync::atomic::{AtomicU64, Ordering};

        static SEQ: AtomicU64 = AtomicU64::new(0);

        pub struct TempDirGuard {
            pub path: PathBuf,
        }

        impl Drop for TempDirGuard {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.path);
            }
        }

        pub fn create(tag: &str) -> TempDirGuard {
            let n = SEQ.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir().join(format!(
                "netpp-test-{}-{}-{}",
                tag,
                std::process::id(),
                n
            ));
            TempDirGuard { path }
        }
    }

    fn sample_entity(id: &str, title: &str, body: &str) -> super::super::content::Entity {
        super::super::content::Entity {
            id: id.into(),
            pack_id: "test".into(),
            kind: "command".into(),
            title: title.into(),
            category: "reference/os/linux/commands".into(),
            tags: vec!["linux".into()],
            risk: Some("low".into()),
            version: None,
            body: body.into(),
        }
    }

    #[test]
    fn settings_roundtrip() {
        let (store, _g) = temp_store("settings");
        assert!(store.setting_get("theme").is_none());
        store.setting_set("theme", "dark").unwrap();
        assert_eq!(store.setting_get("theme"), Some("dark".into()));
        store.setting_set("theme", "light").unwrap();
        assert_eq!(store.setting_get("theme"), Some("light".into()));
    }

    #[test]
    fn favorites_and_pins() {
        let (store, _g) = temp_store("favorites");
        store.favorite_add("a/1", "ping", "command").unwrap();
        store.favorite_add("a/1", "ping", "command").unwrap(); // no dup
        assert_eq!(store.favorites().unwrap().len(), 1);
        assert!(store.is_favorite("a/1"));
        store.favorite_remove("a/1").unwrap();
        assert!(!store.is_favorite("a/1"));

        store.pin_add("tool/cidr", "CIDR", "tool", "/tools?tool=cidr").unwrap();
        let pins = store.pins().unwrap();
        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0].3, "/tools?tool=cidr");
        store.pin_remove("tool/cidr").unwrap();
        assert!(store.pins().unwrap().is_empty());
    }

    #[test]
    fn history_bounded_and_deduped() {
        let (store, _g) = temp_store("history");
        for i in 0..30 {
            store.history_add(&format!("e/{i}"), &format!("E{i}"), "command")
                .unwrap();
        }
        store.history_add("e/1", "E1", "command").unwrap();
        let rows = store.history(100).unwrap();
        assert_eq!(rows.len(), 30); // deduped by id
        store.history_clear().unwrap();
        assert!(store.history(10).unwrap().is_empty());
    }

    #[test]
    fn entity_crud_and_search() {
        let (store, _g) = temp_store("entities");
        let entities = vec![
            sample_entity("cmd/ping", "ping — send ICMP echo", "ping sends ICMP echo requests and reports RTT and loss."),
            sample_entity("cmd/ip-a", "ip a — show addresses", "ip address shows interfaces and addresses on Linux."),
        ];
        store.entities_replace_pack("test", &entities).unwrap();
        assert_eq!(store.stats().unwrap().0, 2);

        let got = store.entity_get("cmd/ping").unwrap().unwrap();
        assert_eq!(got.title, "ping — send ICMP echo");

        let hits = store.search("icmp echo", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "cmd/ping");
        assert!(hits[0].snippet.contains("["));

        // quote-heavy queries must not error
        assert!(store.search("a \"b\" (c)", 10).is_ok());

        store.pack_remove("test").unwrap();
        assert_eq!(store.stats().unwrap().0, 0);
        assert!(store.entity_get("cmd/ping").unwrap().is_none());
    }

    #[test]
    fn categories_and_lists() {
        let (store, _g) = temp_store("categories");
        let mut e = sample_entity("c/1", "one", "body one about routing");
        e.category = "reference/networking/routing".into();
        let mut e2 = sample_entity("c/2", "two", "body two about dns");
        e2.category = "reference/networking/dns".into();
        store.entities_replace_pack("p", &[e, e2]).unwrap();
        let cats = store.categories().unwrap();
        assert_eq!(cats.len(), 2);
        let list = store.entity_list("reference/networking", None).unwrap();
        assert_eq!(list.len(), 2);
    }
}
