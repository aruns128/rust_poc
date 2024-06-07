use reqwest;
use std::error::Error;

pub async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    log::info!("Asynchronous Started");

    match reqwest::get(url).await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    log::info!("Asynchronous Completed");
                    println!("Asynchronous Completed {}", body);
                    Ok(body)
                }
                Err(e) => {
                    log::error!("Failed to extract response text: {}", e);
                    println!("Error: {}", e);
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to send request: {}", e);
            println!("Error: {}", e);
            Err(Box::new(e))
        }
    }
}
