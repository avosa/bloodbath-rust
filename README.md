# bloodbath-rust

This is a Rust implementation of the [Bloodbath](https://docs.bloodbath.io/) event scheduling library. This library is to be moved [here.](https://github.com/bloodbath-io)

## Getting Started

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
bloodbath = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

Here's a basic example of how to use the bloodbath-rust library:

```rust
//[your_project_name]/src/main.rs

use bloodbath::{Bloodbath, Event};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a Bloodbath instance with your API key
    let bloodbath = Arc::new(Bloodbath::new("your_api_key"));

    // Schedule an event
    let args = "time=2023-04-27T12:00:00Z;name=ExampleEvent";
    let result = Event::schedule(&bloodbath, args).await;

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

## API

Bloodbath

- `new(base_url: &str, api_key: &str) -> Bloodbath`: Creates a new Bloodbath instance.

## Event

- `schedule(bloodbath: &Bloodbath, args: &str) -> Result<Event, reqwest::Error>`: Schedules a new event with the provided arguments.

## License

bloodbath-rust is released under the MIT License.
