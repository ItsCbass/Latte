// src/main.rs
use clap::{App, Arg, Command}; 
// Whining: Had to use Clap 3 cus using 4 gives me an error. Fuck my life <3
mod http_client;
mod cli_parser;
mod response;

use std::process;

#[tokio::main]
async fn main() {
    let app = App::new("latte")
        .version("1.0")
        .author("Your Name")
        .about("Does awesome things")
        .subcommand(Command::new("get")
            .about("performs a GET request")
            .arg(Arg::new("url")
                .required(true)
                .help("URL to request")))
        // Add other subcommands here
        ;

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let url = sub_matches.value_of("url").unwrap();
            match http_client::send_request("GET", url).await {
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