mod bloodbath;
mod bloodbath_event;
mod configuration;
mod event;
mod utils;

pub use configuration::Configuration;
pub use event::Event;
pub use bloodbath::Bloodbath;
pub use bloodbath_event::BloodbathEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), reqwest::Error> {
        // Enable debug logging for reqwest crate
        std::env::set_var("RUST_LOG", "reqwest::debug");
        env_logger::init();

        let mut bloodbath = Bloodbath::new("https://api.bloodbath.io/rest");
        bloodbath.set_api_key("test-api-key".to_string());
        bloodbath.set_verbose(true);

        let event = BloodbathEvent::new()
            .scheduled_for((chrono::Utc::now() + chrono::Duration::seconds(10)).timestamp())
            .headers(std::collections::HashMap::new())
            .method("POST".to_string())
            .body("some body content".to_string())
            .endpoint("https://api.acme.com/path".to_string());

        let event_id = bloodbath.schedule_event(&event).await?;

        println!("Event scheduled with ID: {}", event_id);

        let events = bloodbath.list_events().await?;

        println!("Events: {:?}", events);

        let found_event = bloodbath.find_event(&event_id).await?;

        println!("Found event: {:?}", found_event);

        bloodbath.cancel_event(&event_id).await?;

        println!("Event cancelled with ID: {}", event_id);

        Ok(())
    }
}

