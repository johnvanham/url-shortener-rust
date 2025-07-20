use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use log::{info, error};

pub fn init_database() -> SqliteResult<()> {
    let db_path = get_db_path();
    info!("Initializing database at: {}", db_path);
    
    let conn = Connection::open(&db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            key TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    info!("Database initialized successfully");
    Ok(())
}

pub fn get_url_from_db(key: String) -> SqliteResult<String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    
    info!("Looking up URL for key: {}", key);
    
    let mut stmt = conn.prepare("SELECT url FROM urls WHERE key = ?1")?;
    let url: String = stmt.query_row([&key], |row| {
        Ok(row.get(0)?)
    })?;
    
    info!("Found URL for key {}: {}", key, url);
    Ok(url)
}

pub fn set_url_in_db(key: String, url: String) -> SqliteResult<String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    
    info!("Storing URL for key {}: {}", key, url);
    
    conn.execute(
        "INSERT OR REPLACE INTO urls (key, url) VALUES (?1, ?2)",
        [&key, &url],
    )?;
    
    info!("Successfully stored URL for key: {}", key);
    Ok(format!("Stored URL for key: {}", key))
}

fn get_db_path() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "urls.db".to_string())
}