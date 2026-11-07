// Runbook progress — persisted checkbox state per runbook entity.

use super::store::Store;

impl Store {
    pub fn rb_progress_get(&self, entity_id: &str) -> Result<Vec<usize>, String> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS runbook_progress (
                entity_id TEXT PRIMARY KEY,
                checked TEXT NOT NULL DEFAULT '[]',
                updated_at INTEGER NOT NULL
            );",
        )
        .map_err(|e| e.to_string())?;
        let json: String = conn
            .query_row(
                "SELECT checked FROM runbook_progress WHERE entity_id = ?1",
                [entity_id],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "[]".to_string());
        Ok(serde_json::from_str(&json).unwrap_or_default())
    }

    pub fn rb_progress_set(&self, entity_id: &str, checked: &[usize]) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS runbook_progress (
                entity_id TEXT PRIMARY KEY,
                checked TEXT NOT NULL DEFAULT '[]',
                updated_at INTEGER NOT NULL
            );",
        )
        .map_err(|e| e.to_string())?;
        let json = serde_json::to_string(checked).unwrap_or_else(|_| "[]".into());
        conn.execute(
            "INSERT INTO runbook_progress (entity_id, checked, updated_at)
             VALUES (?1, ?2, strftime('%s','now'))
             ON CONFLICT(entity_id) DO UPDATE SET
               checked = ?2, updated_at = strftime('%s','now')",
            rusqlite::params![entity_id, json],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn rb_progress_clear(&self, entity_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM runbook_progress WHERE entity_id = ?1", [entity_id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_store(tag: &str) -> Store {
        let n = SEQ.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "netpp-rb-{tag}-{}-{n}",
            std::process::id()
        ));
        Store::open(&dir).expect("store")
    }

    #[test]
    fn progress_roundtrip() {
        let store = temp_store("rt");
        assert!(store.rb_progress_get("rb/x").unwrap().is_empty());
        store.rb_progress_set("rb/x", &[0, 2, 5]).unwrap();
        assert_eq!(store.rb_progress_get("rb/x").unwrap(), vec![0, 2, 5]);
        store.rb_progress_set("rb/x", &[1]).unwrap();
        assert_eq!(store.rb_progress_get("rb/x").unwrap(), vec![1]);
        store.rb_progress_clear("rb/x").unwrap();
        assert!(store.rb_progress_get("rb/x").unwrap().is_empty());
    }

    #[test]
    fn progress_isolated_per_runbook() {
        let store = temp_store("iso");
        store.rb_progress_set("rb/a", &[1]).unwrap();
        store.rb_progress_set("rb/b", &[2]).unwrap();
        assert_eq!(store.rb_progress_get("rb/a").unwrap(), vec![1]);
        assert_eq!(store.rb_progress_get("rb/b").unwrap(), vec![2]);
    }
}

impl Store {
    /// Wipe ALL application data (settings, favorites, pins, history,
    /// sessions, packs, runbook progress) and reinstall the core pack.
    /// The database file stays in place (open connection); its contents
    /// are destroyed table-by-table instead.
    pub fn factory_reset(&self, packs_root: &std::path::Path) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "DELETE FROM session_results;
             DELETE FROM sessions;
             DELETE FROM history;
             DELETE FROM favorites;
             DELETE FROM pins;
             DELETE FROM settings;
             DELETE FROM entities_fts;
             DELETE FROM entities;
             DELETE FROM packs;
             DELETE FROM runbook_progress;
             VACUUM;",
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        std::fs::remove_dir_all(packs_root).ok();
        Ok(())
    }
}

#[cfg(test)]
mod tests_reset {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ2: AtomicU64 = AtomicU64::new(0);

    fn temp_store(tag: &str) -> Store {
        let n = SEQ2.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "netpp-reset-{tag}-{}-{n}",
            std::process::id()
        ));
        Store::open(&dir).expect("store")
    }

    #[test]
    fn factory_reset_wipes_everything() {
        let store = temp_store("reset");
        store.setting_set("theme", "dark").unwrap();
        store.favorite_add("x/1", "T", "command").unwrap();
        store.history_add("x/1", "T", "command").unwrap();
        let id = store.session_create("s").unwrap();
        store.session_add_result(id, "ping", "1.1.1.1", true, "ok").unwrap();
        store.rb_progress_set("rb/x", &[1]).unwrap();

        let packs_root = std::env::temp_dir().join(format!(
            "netpp-reset-packs-{}-{id}",
            std::process::id()
        ));
        std::fs::create_dir_all(&packs_root).unwrap();
        store.factory_reset(&packs_root).unwrap();

        let (entities, packs, favorites, history) = store.stats().unwrap();
        assert_eq!((entities, packs, favorites, history), (0, 0, 0, 0));
        assert!(store.session_list().unwrap().is_empty());
        assert!(store.pins().unwrap().is_empty());
        assert!(store.setting_get("theme").is_none());
        assert!(store.rb_progress_get("rb/x").unwrap().is_empty());
        assert!(!packs_root.exists());
        let _ = std::fs::remove_dir_all(std::env::temp_dir().join(format!(
            "netpp-reset-reset-{}-{id}",
            std::process::id()
        )));
    }
}
