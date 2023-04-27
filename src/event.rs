use crate::{Bloodbath, BloodbathEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub timestamp: i64,
}

impl Event {
    pub async fn schedule(bloodbath: &Bloodbath, event: &BloodbathEvent) -> Result<Event, reqwest::Error> {
        let url = format!("{}/schedule", bloodbath.api_base);
        let mut request_builder = bloodbath.client.post(&url);

        if let Some(api_key) = &bloodbath.api_key {
            request_builder = request_builder.header("Authorization", api_key);
        }

        let request = request_builder
            .json(event)
            .send()
            .await?;

        let event: Event = request.json().await?;

        Ok(event)
    }

    pub async fn list(bloodbath: &Bloodbath) -> Result<Vec<Event>, reqwest::Error> {
        let url = format!("{}/events", bloodbath.api_base);
        let mut request_builder = bloodbath.client.get(&url);

        if let Some(api_key) = &bloodbath.api_key {
            request_builder = request_builder.header("Authorization", api_key);
        }

        let request = request_builder.send().await?;

        let events: Vec<Event> = request.json().await?;

        Ok(events)
    }

    pub async fn find(bloodbath: &Bloodbath, event_id: &str) -> Result<Event, reqwest::Error> {
        let url = format!("{}/events/{}", bloodbath.api_base, event_id);
        let mut request_builder = bloodbath.client.get(&url);

        if let Some(api_key) = &bloodbath.api_key {
            request_builder = request_builder.header("Authorization", api_key);
        }

        let request = request_builder.send().await?;

        let event: Event = request.json().await?;

        Ok(event)
    }

    pub async fn cancel(bloodbath: &Bloodbath, event_id: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/events/{}", bloodbath.api_base, event_id);
        let mut request_builder = bloodbath.client.delete(&url);

        if let Some(api_key) = &bloodbath.api_key {
            request_builder = request_builder.header("Authorization", api_key);
        }

        let _ = request_builder.send().await?;

        Ok(())
    }
}
