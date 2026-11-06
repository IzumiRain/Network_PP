// Diagnostic sessions — an `impl Store` extension block (Rust allows multiple
// impl blocks per type across modules). Tables: sessions, session_results.

use super::store::Store;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SessionRow {
    pub id: i64,
    pub title: String,
    pub notes: String,
    pub created_at: i64,
    pub result_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionResultRow {
    pub id: i64,
    pub ts: i64,
    pub kind: String,
    pub target: String,
    pub ok: bool,
    pub summary: String,
}

impl Store {
    pub fn session_create(&self, title: &str) -> Result<i64, String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO sessions (title, created_at) VALUES (?1, strftime('%s','now'))",
            [title],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }

    pub fn session_list(&self) -> Result<Vec<SessionRow>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.title, s.notes, s.created_at,
                        (SELECT COUNT(*) FROM session_results r WHERE r.session_id = s.id)
                 FROM sessions s ORDER BY s.created_at DESC",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| {
                Ok(SessionRow {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    notes: r.get(2)?,
                    created_at: r.get(3)?,
                    result_count: r.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn session_add_result(
        &self,
        session_id: i64,
        kind: &str,
        target: &str,
        ok: bool,
        summary: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO session_results (session_id, ts, kind, target, ok, summary)
             VALUES (?1, strftime('%s','now'), ?2, ?3, ?4, ?5)",
            rusqlite::params![session_id, kind, target, ok, summary],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn session_results(&self, session_id: i64) -> Result<Vec<SessionResultRow>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, ts, kind, target, ok, summary FROM session_results
                 WHERE session_id = ?1 ORDER BY ts, id",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([session_id], |r| {
                Ok(SessionResultRow {
                    id: r.get(0)?,
                    ts: r.get(1)?,
                    kind: r.get(2)?,
                    target: r.get(3)?,
                    ok: r.get::<_, i64>(4)? != 0,
                    summary: r.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(rows)
    }

    pub fn session_set_notes(&self, session_id: i64, notes: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET notes = ?1 WHERE id = ?2",
            rusqlite::params![notes, session_id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn session_delete(&self, session_id: i64) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM sessions WHERE id = ?1", [session_id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn session_export_md(&self, session_id: i64) -> Result<String, String> {
        let conn = self.conn.lock().unwrap();
        let (title, notes, created): (String, String, i64) = conn
            .query_row(
                "SELECT title, notes, created_at FROM sessions WHERE id = ?1",
                [session_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| format!("session {session_id} not found"))?;
        let rows: Vec<(i64, String, String, i64, String)> = {
            let mut stmt = conn
                .prepare(
                    "SELECT ts, kind, target, ok, summary FROM session_results
                     WHERE session_id = ?1 ORDER BY ts, id",
                )
                .map_err(|e| e.to_string())?;
            let mapped = stmt.query_map([session_id], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, String>(4)?,
                ))
            });
            let collected = mapped
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;
            collected
        };

        let iso = |unix: i64| {
            crate::commands::convert::timestamp_from_unix(unix).iso_utc
        };
        let mut md = format!(
            "# Diagnostic session — {title}\n\n- Started: {}\n- Results: {}\n",
            iso(created),
            rows.len()
        );
        if !notes.is_empty() {
            md.push_str(&format!("\n## Notes\n\n{notes}\n"));
        }
        md.push_str("\n| Time | Test | Target | Result | Detail |\n|---|---|---|---|---|\n");
        for (ts, kind, target, ok, summary) in rows {
            let flag = if ok == 1 { "OK" } else { "FAIL" };
            let target_md = target.replace('|', "\\|");
            let summary_md = summary.replace('|', "\\|");
            md.push_str(&format!(
                "| {} | {kind} | {target_md} | {flag} | {summary_md} |\n",
                iso(ts)
            ));
        }
        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_store(tag: &str) -> Store {
        let n = SEQ.fetch_add(1, Ordering::SeqCst);
        let dir = PathBuf::from(std::env::temp_dir()).join(format!(
            "netpp-sessions-{tag}-{}-{n}",
            std::process::id()
        ));
        Store::open(&dir).expect("open store")
    }

    #[test]
    fn session_lifecycle() {
        let store = temp_store("life");
        let id = store.session_create("Edge router check").unwrap();
        store.session_add_result(id, "ping", "10.0.0.1", true, "12 ms").unwrap();
        store.session_add_result(id, "dns", "example.com A", false, "NXDOMAIN").unwrap();
        store.session_set_notes(id, "Checked after firmware update").unwrap();

        let list = store.session_list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].result_count, 2);
        assert_eq!(list[0].notes, "Checked after firmware update");

        let results = store.session_results(id).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].ok);
        assert!(!results[1].ok);

        let md = store.session_export_md(id).unwrap();
        assert!(md.contains("# Diagnostic session — Edge router check"));
        assert!(md.contains("| ping | 10.0.0.1 | OK | 12 ms |"));
        assert!(md.contains("## Notes"));

        store.session_delete(id).unwrap();
        assert!(store.session_list().unwrap().is_empty());
        // cascade removed results too
        assert!(store.session_results(id).unwrap().is_empty());
    }

    #[test]
    fn export_escapes_pipes() {
        let store = temp_store("esc");
        let id = store.session_create("t").unwrap();
        store.session_add_result(id, "tcp", "a|b", true, "open|filtered").unwrap();
        let md = store.session_export_md(id).unwrap();
        assert!(md.contains("open\\|filtered"));
        assert!(md.contains("a\\|b"));
    }

    #[test]
    fn export_missing_session() {
        let store = temp_store("missing");
        assert!(store.session_export_md(999).is_err());
    }
}
