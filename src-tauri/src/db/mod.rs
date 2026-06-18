use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};
use std::path::PathBuf;
use std::str::FromStr;

pub type DbPool = SqlitePool;

pub async fn init_db(app_data_dir: PathBuf) -> Result<DbPool, Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("paper-manager.sqlite");

    println!("App data dir: {}", app_data_dir.display());
    println!("Database path: {}", db_path.display());

    let connect_options = SqliteConnectOptions::from_str(&format!(
        "sqlite:{}",
        db_path.to_string_lossy()
    ))?
    .create_if_missing(true)
    .journal_mode(SqliteJournalMode::Wal)
    .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./src/db/migrations").run(&pool).await?;

    Ok(pool)
}