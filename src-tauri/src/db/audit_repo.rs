use anyhow::Result;
use rusqlite::Connection;

use crate::models::audit::AuditEntry;

pub fn insert_entry(conn: &Connection, entry: &AuditEntry) -> Result<()> {
    conn.execute(
        "INSERT INTO audit_log (item_path, display_name, category, size_bytes, deleted_at, success, error_message)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            entry.item_path,
            entry.item_display_name,
            entry.category,
            entry.size_bytes,
            entry.deleted_at,
            entry.success,
            entry.error_message,
        ],
    )?;
    Ok(())
}

pub fn list_entries(conn: &Connection, limit: i64, offset: i64) -> Result<Vec<AuditEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, item_path, display_name, category, size_bytes, deleted_at, success, error_message
         FROM audit_log ORDER BY deleted_at DESC LIMIT ?1 OFFSET ?2",
    )?;
    let entries = stmt
        .query_map(rusqlite::params![limit, offset], |row| {
            Ok(AuditEntry {
                id: row.get(0)?,
                item_path: row.get(1)?,
                item_display_name: row.get(2)?,
                category: row.get(3)?,
                size_bytes: row.get::<_, i64>(4)? as u64,
                deleted_at: row.get(5)?,
                success: row.get(6)?,
                error_message: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(entries)
}
