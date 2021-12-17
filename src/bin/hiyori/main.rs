use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, App, Arg,
};

use hiyori::openweather::client::Client;
use std::{env, process::exit};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("location")
                .long("location")
                .short("l")
                .default_value("tokyo")
                .help("Weather location"),
        )
        .get_matches();

    let api_key = match env::var("OPENWEATHER_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("OPENWEATHER_API_KEY required.");
            exit(1);
        }
    };

    let location = value_t_or_exit!(matches.value_of("location"), String);

    let client = Client::new(&api_key, "metric");
    let current_weather = match client.weather(&location) {
        Ok(weather) => weather,
        Err(_) => {
            eprintln!("Request failed");
            exit(1);
        }
    };

    println!("Location name: {}", current_weather.name);
    println!("Temp: {}", current_weather.main.temp);
    println!("Pressure: {}", current_weather.main.pressure);
    println!("Humidity: {}", current_weather.main.humidity);
}
