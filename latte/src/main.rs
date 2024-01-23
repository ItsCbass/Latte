// src/main.rs
mod http_client;
mod cli_parser;
mod response;

fn main() {
    let args = cli_parser::parse_args();
    println!("Method: {}, URL: {}", args.method, args.url);
    // CLI parsing and HTTP request handling
}