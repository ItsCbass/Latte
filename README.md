# Latte 
A Rust-based commmand-line HTTP client inspired by HTTPie. I've fallen in love with Rust recently so why not make something actually useful in it. As of 1.23.2024, we've started the project and implemented the first few features like GET requests.

# Features
- Fast and efficient HTTP requests using Rust's async functionality
- Colorful output for asethetics
- Pretty printing of JSON responses (SOON)
- Customizable and extensible functionality - Adding headers, queries, etc.
- Support various HTTP methods including GET, POST, PUT, and DELETE (SOON)

# Installation
```bash
git clone https://github.com/your-username/httpie-clone.git
cd httpie-clone
cargo build --release
```

# Usage
Perform a simple GET requrest:
```bash
./target/release/latte get 'https://api.example.com'
```