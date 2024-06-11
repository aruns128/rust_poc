extern crate redis;

use redis::AsyncCommands;
use std::error::Error;
use dotenv::dotenv;
use std::env;

pub async fn set_value(con: &mut redis::aio::Connection, key: &str, value: &str) -> redis::RedisResult<()> {
    con.set(key, value).await?;
    log::info!("Set '{}' to '{}'", key, value);
    Ok(())
}

pub async fn get_value(con: &mut redis::aio::Connection, key: &str) -> redis::RedisResult<Option<String>> {
    let result: Option<String> = con.get(key).await?;
    match &result {
        Some(value) => {log::info!("{}: {}", key, value);},
        None => {log::error!("{} does not exist", key);},
    }
    Ok(result)
}

pub async fn delete_value(con: &mut redis::aio::Connection, key: &str) -> redis::RedisResult<()> {
    con.del(key).await?;
    log::warn!("Deleted '{}'", key);
    Ok(())
}

pub async fn handle_redis() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load .env file if it exists
    // Connect to the Redis server
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    // Create: Set a key-value pair
    set_value(&mut con, "test_key", "Hello, Redis!").await?;

    // Read: Get the value of a key
    get_value(&mut con, "test_key").await?;

    // Update: Update the value of an existing key
    set_value(&mut con, "test_key", "Hello world, Redis!").await?;

    // Read: Get the updated value of the key
    get_value(&mut con, "test_key").await?;

    // Delete: Remove the key
    delete_value(&mut con, "test_key").await?;

    // Read: Try to get the value of the deleted key
    get_value(&mut con, "test_key").await?;

    Ok(())
}
