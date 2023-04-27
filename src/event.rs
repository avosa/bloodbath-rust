use crate::Bloodbath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub timestamp: i64,
}

impl Event {
    pub async fn schedule(bloodbath: &Bloodbath, args: &str) -> Result<Event, reqwest::Error> {
        let url = format!("{}/schedule?args={}", bloodbath.base_url, args);
        let response = bloodbath.client.get(&url).send().await?;
        let event: Event = response.json().await?;
        Ok(event)
    }
}
