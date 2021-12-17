use http::StatusCode;
use reqwest;
use serde_json;

use super::types::CurrentWeather;

pub struct Client {
    base_url: String,
    api_key: String,
    units: String,
}

impl Client {
    pub fn new(api_key: &str, units: &str) -> Self {
        Client {
            base_url: "https://api.openweathermap.org/data/2.5/weather".to_string(),
            api_key: api_key.to_string(),
            units: units.to_string(),
        }
    }

    fn fetch(&self, query_string: &str) -> Result<String, String> {
        let url = format!(
            "{}?{}&units={}&appid={}",
            self.base_url, query_string, self.units, self.api_key
        );

        match reqwest::blocking::get(url) {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response.text().unwrap()),
                _ => Err("error".to_string()),
            },
            Err(_) => Err("error".to_string()),
        }
    }

    pub fn weather(&self, location: &str) -> Result<CurrentWeather, String> {
        let query_string = format!("q={}", location);

        match self.fetch(&query_string) {
            Ok(body) => match serde_json::from_str(&body) {
                Ok(current_weather) => Ok(current_weather),
                Err(e) => Err(format!("{}", e)),
            },
            Err(_) => Err("request error".to_string()),
        }
    }
}
