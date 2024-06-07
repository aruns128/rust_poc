use std::env;

mod rest_api;
mod logging;
mod async_operation;
mod work_with_json;

#[tokio::main]
async fn main() {
   logging::init_log();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() < 2 {
        log::info!("REST program started without arguments");
        println!("Usage: cargo run [database]");
        println!("Example: cargo run postgres, the available actions are rest_api, async_operation, json_example");
        return;
    }

    match args[1].as_str() {
        "rest_api" => {
            log::info!("Starting REST CRUD operation");
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
        _ => log::warn!("Unsupported type."),
    }
}
