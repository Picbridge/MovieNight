use dotenv::dotenv;
use std::env;

lazy_static::lazy_static! {
    pub static ref APIBASE: String = {
        dotenv().ok(); // Load the .env file once
        let api_base = env::var("APIBASE").unwrap_or_else(|_| "http://localhost:5173/api".to_string());
        log::info!("Loaded APIBASE: {}", api_base);
        api_base
    };
}

/// Provides the APIBASE as a &'static str for consistent usage.
pub fn api_base() -> &'static str {
    //APIBASE
    "https://movienight-backend.purplebay-46d91d82.westus2.azurecontainerapps.io/api"
    //"http://localhost:5173/api"
}

pub fn logo_src() -> &'static str {
    "https://github.com/Picbridge/MovieNight/blob/main/logo.png?raw=true"
}
