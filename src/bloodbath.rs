use crate::event::Event;
use crate::BloodbathEvent;
use reqwest::Client;

pub struct Bloodbath {
    pub base_url: String,
    pub client: Client,
    pub api_key: Option<String>,
    pub api_base: String,
    pub verbose: bool,
}

impl Bloodbath {
    pub fn new(base_url: &str) -> Bloodbath {
        Bloodbath {
            base_url: base_url.to_string(),
            client: Client::new(),
            api_key: None,
            api_base: "https://api.bloodbath.io/rest".to_string(),
            verbose: false,
        }
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    pub fn set_api_base(&mut self, api_base: String) {
        self.api_base = api_base;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub async fn schedule_event(&self, event: &BloodbathEvent) -> Result<String, reqwest::Error> {
        Event::schedule(self, &event).await.map(|event| event.id)
    }

    pub async fn list_events(&self) -> Result<Vec<Event>, reqwest::Error> {
        Event::list(self).await
    }

    pub async fn find_event(&self, event_id: &str) -> Result<Event, reqwest::Error> {
        Event::find(self, event_id).await
    }

    pub async fn cancel_event(&self, event_id: &str) -> Result<(), reqwest::Error> {
        Event::cancel(self, event_id).await.map(|_| ())
    }
}
