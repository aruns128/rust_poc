use dotenv::dotenv;
use polars::error::PolarsError;
use polars::prelude::*;
use std::env;
use thiserror::Error;
use tokio_postgres::{Error as PostgresError, NoTls}; // For custom error handling

// Define a custom error type
#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Postgres error: {0}")]
    PgError(#[from] PostgresError),

    #[error("Polars error: {0}")]
    PolarsError(#[from] PolarsError),
}

pub async fn fetch_data() -> Result<(), CustomError> {
    dotenv().ok();

    log::info!("Loading environment variables...");

    let db_host = env::var("DB_HOST")?;
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| {
        warn!("DB_PORT not set, defaulting to 5432.");
        "5432".to_string()
    });
    let db_user = env::var("DB_USER")?;
    let db_password = env::var("DB_PASSWORD")?;
    let db_name = env::var("DB_NAME")?;

    let database_url = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_password, db_name
    );

    log::info!("Connecting to the database...");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Connection error: {}", e);
        }
    });

    log::info!("Connected to the database.");

    log::info!("Creating table 'person' if not exists...");
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS person (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            data BYTEA
        )",
            &[],
        )
        .await?;

    log::info!("Fetching data from table 'person'...");
    let stmt = "SELECT id, name, data FROM person";
    let rows = client.query(stmt, &[]).await?;

    log::info!("Fetched {} rows.", rows.len());

    let ids: Vec<i32> = rows.iter().map(|row| row.get(0)).collect();
    let names: Vec<String> = rows.iter().map(|row| row.get(1)).collect();
    let data: Vec<Vec<u8>> = rows.iter().map(|row| row.get(2)).collect();

    log::info!("Converting data to Polars DataFrame...");
    let df = match DataFrame::new(vec![
        Series::new("id", ids),
        Series::new("name", names),
        Series::new("data", data),
    ]) {
        Ok(df) => df,
        Err(e) => {
            log::error!("Failed to create DataFrame: {}", e);
            return Err(CustomError::PolarsError(e)); // Convert to the custom error type
        }
    };

    log::info!("DataFrame created successfully.");
    println!("{:?}", df);

    Ok(())
}
