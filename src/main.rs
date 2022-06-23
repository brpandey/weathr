use std::error::Error;

use dotenv::dotenv;
use std::env;

use weathr::request::{WeatherApi, Location, Units};
use weathr::backend::WeatherList;
use weathr::display::WeatherForecast;

fn main() -> Result<(), Box<dyn Error>> {

    // Use environment variable as source of api key via export WEATHER_API_KEY="mykey"
    dotenv().ok();

    let api_key: String = env::var("WEATHER_API_KEY").expect("export WEATHER_API_KEY=apikey not set");
    let api = WeatherApi::new(api_key, Location::City("Many Farms".to_string()), Units::Imperial);
    let response = api.request().unwrap();

    let data: WeatherList = WeatherList::parse(&response)?;
    let forecast: WeatherForecast = data.into();

    println!("{}", forecast);

    Ok(())
}

