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
