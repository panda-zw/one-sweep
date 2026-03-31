use anyhow::Result;
use rusqlite::Connection;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{CategoryResult, ScanCategory, ScanItem, ScanResult};

pub fn upsert_items(conn: &Connection, items: &[ScanItem], scanned_at: i64) -> Result<()> {
    conn.execute("DELETE FROM scan_cache", [])?;

    let mut stmt = conn.prepare(
        "INSERT INTO scan_cache (item_id, path, display_name, description, category, safety, size_bytes, last_modified, scanned_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    for item in items {
        stmt.execute(rusqlite::params![
            item.id,
            item.path,
            item.display_name,
            item.description,
            item.category.as_str(),
            item.safety.as_str(),
            item.size_bytes as i64,
            item.last_modified,
            scanned_at,
        ])?;
    }

    conn.execute(
        "INSERT OR REPLACE INTO metadata (key, value) VALUES ('last_scan_at', ?1)",
        rusqlite::params![scanned_at.to_string()],
    )?;

    Ok(())
}

pub fn get_cached_scan(conn: &Connection) -> Result<Option<ScanResult>> {
    let last_scan_at: Option<String> = conn
        .query_row(
            "SELECT value FROM metadata WHERE key = 'last_scan_at'",
            [],
            |row| row.get(0),
        )
        .ok();

    let scanned_at = match last_scan_at {
        Some(s) => s.parse::<i64>().unwrap_or(0),
        None => return Ok(None),
    };

    let mut stmt = conn.prepare(
        "SELECT item_id, path, display_name, description, category, safety, size_bytes, last_modified
         FROM scan_cache ORDER BY size_bytes DESC",
    )?;

    let items: Vec<ScanItem> = stmt
        .query_map([], |row| {
            Ok(ScanItem {
                id: row.get(0)?,
                path: row.get(1)?,
                display_name: row.get(2)?,
                description: row.get(3)?,
                category: ScanCategory::from_str(&row.get::<_, String>(4)?),
                safety: SafetyLevel::from_str(&row.get::<_, String>(5)?),
                size_bytes: row.get::<_, i64>(6)? as u64,
                last_modified: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if items.is_empty() {
        return Ok(None);
    }

    let result = build_scan_result(items, scanned_at);
    Ok(Some(result))
}

pub fn get_cached_items(conn: &Connection) -> Result<Vec<ScanItem>> {
    let mut stmt = conn.prepare(
        "SELECT item_id, path, display_name, description, category, safety, size_bytes, last_modified
         FROM scan_cache",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(ScanItem {
                id: row.get(0)?,
                path: row.get(1)?,
                display_name: row.get(2)?,
                description: row.get(3)?,
                category: ScanCategory::from_str(&row.get::<_, String>(4)?),
                safety: SafetyLevel::from_str(&row.get::<_, String>(5)?),
                size_bytes: row.get::<_, i64>(6)? as u64,
                last_modified: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(items)
}

fn build_scan_result(items: Vec<ScanItem>, started_at: i64) -> ScanResult {
    use std::collections::HashMap;

    let mut category_map: HashMap<String, Vec<ScanItem>> = HashMap::new();
    for item in items {
        let key = item.category.as_str().to_string();
        category_map.entry(key).or_default().push(item);
    }

    let mut categories: Vec<CategoryResult> = category_map
        .into_iter()
        .map(|(_key, items)| {
            let category = items[0].category.clone();
            let total_bytes: u64 = items.iter().map(|i| i.size_bytes).sum();
            CategoryResult {
                display_name: category.display_name().to_string(),
                description: category.description().to_string(),
                category,
                total_bytes,
                items,
            }
        })
        .collect();

    categories.sort_by(|a, b| b.total_bytes.cmp(&a.total_bytes));

    let total_bytes: u64 = categories.iter().map(|c| c.total_bytes).sum();

    ScanResult {
        started_at,
        completed_at: Some(started_at),
        total_bytes,
        categories,
    }
}
