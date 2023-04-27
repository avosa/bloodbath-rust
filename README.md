# bloodbath-rust

This is a Rust implementation of the [Bloodbath](https://github.com/Loschcode/bloodbath-api) event scheduling library.

## Getting Started

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
bloodbath-rust = { git = "https://github.com/avosa/bloodbath-rust.git" }
```

# Usage Example

Here's a basic example of how to use the bloodbath-rust library:

```rust
use bloodbath_rust::{Bloodbath, Event};
use std::sync::Arc;

async fn main() {
    // Create a Bloodbath instance with your API key
    let bloodbath = Arc::new(Bloodbath::new("your_api_key"));

    // Schedule an event
    let args = "time=2023-04-27T12:00:00Z;name=ExampleEvent";
    let result = Event::schedule(&bloodbath, args).await;

    match result {
        Ok(event) => println!("Event scheduled: {:?}", event),
        Err(e) => eprintln!("Error scheduling event: {:?}", e),
    }
}

```

Replace "your_api_key" with your Bloodbath API key.

## API

Bloodbath

- `new(base_url: &str, api_key: &str) -> Bloodbath`: Creates a new Bloodbath instance.

## Event

- `schedule(bloodbath: &Bloodbath, args: &str) -> Result<Event, reqwest::Error>`: Schedules a new event with the provided arguments.

## License

bloodbath-rust is released under the MIT License. See LICENSE for details.
