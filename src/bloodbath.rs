use crate::event::Event;
use reqwest::Client;

pub struct Bloodbath {
    pub base_url: String,
    pub client: Client,
}

impl Bloodbath {
    pub fn new(base_url: &str) -> Bloodbath {
        Bloodbath {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn schedule_event(&self, args: &str) -> Result<Event, reqwest::Error> {
        Event::schedule(self, args).await
    }
}
