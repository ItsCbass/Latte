// src/http_client.rs
use reqwest::{self};


pub async fn send_request(method: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = match method {
        "GET" => client.get(url).send().await?,
        // Add other methods like POST, PUT, DELETE here
        _ => panic!("Unsupported method!"),
    };

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    Ok(format!("Status: {}\nHeaders: {:?}\nBody:\n{}", status, headers, body))
}