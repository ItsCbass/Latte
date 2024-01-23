// src/cli_parser.rs
use clap::{App, Arg, Subcommand};

pub struct CommandLineArgs {
    pub method: String,
    pub url: String,
    // Add more fields
}

pub fn parse_args() -> CommandLineArgs { 
    let matches = App::new("latte")
        .version("0.1")
        .author("Sebastian Rivera")
        .about("HTTP Client")
        .subcommand(Subcommand::with_name("get")
            .about("performs a GET request")
            .arg(Arg::with_name("url")
                .required(true)
                .help("URL to request")))
        // Add other HTTP methods
        .get_matches();

    let (method, url) = if let Some(matches) = matches.subcommand_matches("gets") {
        ("GET", matches.value_of("url").unwrap())
    } else {
        // Handle other methods
        panic!("Unsupported HTTP method");
    };

    CommandLineArgs {
        method: method.to_string(),
        url: url.to_string(),
    }
}