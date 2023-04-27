mod bloodbath;
mod configuration;
mod event;
mod utils;
pub use configuration::Configuration;
pub use event::Event;
pub use bloodbath::Bloodbath;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bloodbath = Bloodbath::new();
        bloodbath.set_api_key("test-api-key".to_string());
        bloodbath.set_api_base("https://api.bloodbath.io/rest".to_string());
        bloodbath.set_verbose(true);

        let event = bloodbath.event();
        let _ = event.list().unwrap();
    }
}
