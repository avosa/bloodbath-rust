# bloodbath-rust

This is a Rust implementation of the [Bloodbath](https://docs.bloodbath.io/) event scheduling library. This library is to be moved [here.](https://github.com/bloodbath-io)

## Getting Started

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
bloodbath = "0.1.3"
tokio = { version = "1", features = ["full"] }
```

Or run the following command to add the dependency to your project's `Cargo.toml` file:

```bash
cargo add bloodbath
```

## Usage

Here's a basic example of how to use the bloodbath-rust library:

```rust
//[your_project_name]/src/main.rs

use bloodbath::{Bloodbath, Event, BloodbathEvent};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a Bloodbath instance with your API key
    let bloodbath = Arc::new(Bloodbath::new("your_api_key"));

    // Schedule an event
    let timestamp_str = "2023-04-27T12:00:00Z";
    let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str)
        .unwrap()
        .timestamp();
    let event = BloodbathEvent {
        scheduled_for: timestamp,
        headers: Default::default(),
        method: "POST".to_owned(), // or simply Default::default(),
        body: "some body content".to_owned(), // or simply Default::default(),
        endpoint: "https://api.acme.com/path".to_owned(),
    };
    let result = Event::schedule(&bloodbath, &event).await;

    match result {
        Ok(event) => {
            println!("Event scheduled successfully:");
            println!("ID: {}", event.id);
            println!("Name: {}", event.name);
            println!("Timestamp: {}", event.timestamp);
        },
        Err(e) => eprintln!("Error scheduling event: {:?}", e),
    }
}
```

Replace "your_api_key" with your Bloodbath API key.

**IMPORTANT:** To use latest unpublished version direct from source, add to your `Cargo.toml` file:

```toml
[dependencies]
bloodbath = { git = "https://github.com/avosa/bloodbath-rust.git" }
tokio = { version = "1", features = ["full"] }
```

Testing:

```bash
cargo test --lib
```

## Conclusion

Bloodbath is a powerful library that simplifies event scheduling and management. For more information, see the [official documentation](https://docs.bloodbath.io/).
