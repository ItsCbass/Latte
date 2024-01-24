// src/http_client.rs
use reqwest::{header::HeaderMap, Client, Error, Method};

pub async fn send_request(
    method_str: &str,
    url: &str,
    headers: Option<HeaderMap>,
    body: Option<String>,
) -> Result<String, Error> {
    let client = Client::new();
    let method = method_str.parse::<Method>().expect("Invalid HTTP method");

    let mut request_builder = client.request(method, url);

    if let Some(header_map) = headers {
        request_builder = request_builder.headers(header_map);
    }

    if let Some(body_content) = body {
        request_builder = request_builder.body(body_content);
    }

    let response = request_builder.send().await?;
    let status = response.status();
    let response_headers = response.headers().clone();
    let response_body = response.text().await?;

    Ok(format!(
        "Status: {}\nHeaders: {:?}\nBody:\n{}",
        status, response_headers, response_body
    ))
}
