#[derive(Clone, Debug)]
pub struct Configuration {
    pub api_key: Option<String>,
    pub api_base: String,
    pub verbose: bool,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            api_key: None,
            api_base: "https://api.bloodbath.io/rest".to_string(),
            verbose: false,
        }
    }
}
