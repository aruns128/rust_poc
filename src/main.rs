use std::env;

mod rest_api;
mod logging;
mod async_operation;
mod work_with_json;
mod kafka;
mod redis;

#[tokio::main]
async fn main() {
    logging::init_log();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() < 2 {
        log::info!("Program started without arguments");
        println!("Usage: cargo run [action]");
        println!("Example: cargo run redis, the available actions are rest_api, async_operation, json_example, kafka_producer, kafka_consumer, redis");
        return;
    }

    match args[1].as_str() {
        "rest_api" => {
            log::info!("Starting {} operation", args[1].as_str());
            if let Err(e) = rest_api::rest_crud().await {
                log::error!("REST CRUD Operation failed to start: {}", e);
            }
        },
        "async_operation" => {
            let url = "https://jsonplaceholder.typicode.com/posts/1";
            log::info!("Starting Async Operation");
            if let Err(e) = async_operation::fetch_url(url).await {
                log::error!("Async Operation failed to start: {}", e);
            }
        },
        "json_example" => {
            log::info!("Starting JSON Example with rust");
            if let Err(e) = work_with_json::get_person_details().await {
                log::error!("JSON Example with rust failed to start: {}", e);
            }
        },
        "kafka" => {
            let kafka_url = match env::var("KAFKA_URL") {
                Ok(url) => url,
                Err(_) => {
                    log::error!("KAFKA_URL environment variable not set.");
                    return;
                }
            };

            log::info!("Starting Kafka service");
            if let Err(e) = kafka::handle_kafka(&kafka_url).await {
                log::error!("Kafka service failed to start: {}", e);
            }
        },
        "redis" => {
            log::info!("Starting Redis CRUD operation");
            if let Err(e) = redis::redis_operations::handle_redis().await {
                log::error!("Redis CRUD Operation failed to start: {}", e);
            }
        },
        _ => log::warn!("Unsupported Action type."),
    }
}
