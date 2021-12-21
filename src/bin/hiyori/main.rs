use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, App, Arg,
};

use chrono::{DateTime, FixedOffset, TimeZone};

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
    let cw = match client.weather(&location) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Request failed: {}", e);
            exit(1);
        }
    };

    let dt: DateTime<FixedOffset> = FixedOffset::east(cw.timezone).timestamp(cw.dt, 0);

    println!("Location      : {}, {} ({})", cw.name, cw.sys.country, dt);
    if let Some(weather) = cw.weather.iter().nth(0) {
        println!("Weather       : {}, {}", weather.main, weather.description);
    }
    println!("Temperature   : {} C", cw.main.temp);
    println!("Pressure      : {} hPa", cw.main.pressure);
    println!("Humidity      : {} %", cw.main.humidity);
    println!("Wind Speed    : {} m/s", cw.wind.speed);
    println!("Wind Direction: {} deg", cw.wind.deg);
}
