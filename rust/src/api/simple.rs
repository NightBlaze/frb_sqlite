use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::path::Path;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn configure_database(documents_dir: String) {
    let database_url = Path::new(&documents_dir).join("sqlite.db").display().to_string();
    create_database(&database_url).await;

    let pool = SqlitePool::connect(&database_url).await.expect("Fail to create connection pool");

    create_table("table_name", &pool).await;
}

async fn create_database(path: &str) {
    match Sqlite::database_exists(path).await {
        Ok(exists) => {
            if exists {
                return;
            }
            match Sqlite::create_database(path).await {
                Ok(_) => return,
                Err(err) => {
                    panic!("Can't create database. Path: {}. Error: {}", path, err);
                },
            }
        },
        Err(err) => {
            panic!("Can't check database existence. Path: {}. Error: {}", path, err);
        },
    }
}

async fn create_table(table_name: &str, pool: &Pool<Sqlite>) {
    let sql = format!("CREATE TABLE {} (id INTEGER PRIMARY KEY, name TEXT NOT NULL);", &table_name);
    match sqlx::query(&sql).execute(pool).await {
        Ok(_) => return,
        Err(error) => panic!("fail to execute sql. Error: {}", error),
    }
}