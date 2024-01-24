use clap::{App, Arg, Command};
use reqwest::header::{HeaderMap, HeaderValue};
use std::str::FromStr;

mod http_client;

#[tokio::main]
async fn main() {
    let app = App::new("latte")
        .version("0.1")
        .author("Sebastian Rivera")
        .about("Does awesome things")
        .subcommand(Command::new("get")
            .about("performs a GET request")
            .arg(Arg::new("url")
                .required(true)
                .help("URL to request"))
            .arg(Arg::new("headers")
                .multiple_values(true)
                .help("Custom request headers")))
        // Add other subcommands here
        ;

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let url = sub_matches.value_of("url").unwrap();
            let headers = parse_headers(
                sub_matches
                    .values_of("headers")
                    .unwrap_or_default()
                    .collect(),
            );

            match http_client::send_request("GET", url, headers, None).await {
                Ok(response) => println!("{}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(("post", sub_matches)) => {
            let url = sub_matches.value_of("url").unwrap();
            let body = sub_matches.value_of("body").unwrap_or_default(); // Assuming JSON body for simplicity
            let headers = parse_headers(
                sub_matches
                    .values_of("headers")
                    .unwrap_or_default()
                    .collect(),
            );

            match http_client::send_request("POST", url, headers, Some(body.to_string())).await {
                Ok(response) => println!("{}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        // Handle other subcommands or default case
        _ => {
            println!("Unknown command");
        }
    }
}

fn parse_headers(headers_values: Vec<&str>) -> Option<HeaderMap> {
    let mut headers = HeaderMap::new();
    for header_value in headers_values {
        let parts: Vec<&str> = header_value.splitn(2, ':').collect();
        if parts.len() == 2 {
            // Convert header name to a String to own the data
            let header_name = parts[0].trim().to_string();
            let header_value = parts[1].trim();

            headers.insert(
                reqwest::header::HeaderName::from_str(&header_name).expect("Invalid header name"),
                HeaderValue::from_str(header_value).expect("Invalid header value"),
            );
        } else {
            eprintln!("Invalid header format: {}", header_value);
            return None;
        }
    }

    if headers.is_empty() {
        None
    } else {
        Some(headers)
    }
}
