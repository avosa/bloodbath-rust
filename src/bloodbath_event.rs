use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BloodbathEvent {
    pub scheduled_for: i64,
    pub headers: HashMap<String, String>,
    pub method: String,
    pub body: String,
    pub endpoint: String,
}

impl BloodbathEvent {
    pub fn new() -> BloodbathEvent {
        BloodbathEvent {
            scheduled_for: 0,
            headers: HashMap::new(),
            method: "POST".to_string(),
            body: "".to_string(),
            endpoint: "".to_string(),
        }
    }

    pub fn scheduled_for(mut self, scheduled_for: i64) -> BloodbathEvent {
        self.scheduled_for = scheduled_for;
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> BloodbathEvent {
        self.headers = headers;
        self
    }

    pub fn method(mut self, method: String) -> BloodbathEvent {
        self.method = method;
        self
    }

    pub fn body(mut self, body: String) -> BloodbathEvent {
        self.body = body;
        self
    }

    pub fn endpoint(mut self, endpoint: String) -> BloodbathEvent {
        self.endpoint = endpoint;
        self
    }
}
