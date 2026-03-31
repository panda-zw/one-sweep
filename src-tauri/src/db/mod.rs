pub mod audit_repo;
pub mod scan_cache_repo;

use anyhow::Result;
use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS audit_log (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            item_path       TEXT NOT NULL,
            display_name    TEXT NOT NULL,
            category        TEXT NOT NULL,
            size_bytes      INTEGER NOT NULL,
            deleted_at      INTEGER NOT NULL,
            success         INTEGER NOT NULL DEFAULT 1,
            error_message   TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_audit_log_deleted_at ON audit_log(deleted_at);
        CREATE INDEX IF NOT EXISTS idx_audit_log_category ON audit_log(category);

        CREATE TABLE IF NOT EXISTS scan_cache (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id         TEXT NOT NULL UNIQUE,
            path            TEXT NOT NULL,
            display_name    TEXT NOT NULL,
            description     TEXT NOT NULL,
            category        TEXT NOT NULL,
            safety          TEXT NOT NULL,
            size_bytes      INTEGER NOT NULL,
            last_modified   INTEGER,
            scanned_at      INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_scan_cache_category ON scan_cache(category);

        CREATE TABLE IF NOT EXISTS metadata (
            key             TEXT PRIMARY KEY,
            value           TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
