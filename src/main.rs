use std::env;
use std::error::Error;
use dotenv::dotenv;

use weathr::config::WeatherConfig;
use weathr::request::WeatherApi;
use weathr::backend::WeatherList;
use weathr::display::WeatherForecast;

fn main() -> Result<(), Box<dyn Error>> {

    // Use environment variable as source of api key via export WEATHER_API_KEY="mykey"
    // 12 factor app - best practices "store config in environment"
    dotenv().ok();

    let api_key: String = env::var("WEATHER_API_KEY").expect("export WEATHER_API_KEY= not set");

    // Retrieve user supplied config portion
    let config = WeatherConfig::load();

    // Construct api request
    let api = WeatherApi::load(api_key, config.location(), config.units())?;

    let response = api.request()?;

    // Parse then normalize data before displaying
    let data: WeatherList = WeatherList::parse(&response)?;
    let forecast: WeatherForecast = data.into();

    println!("{}", forecast);

    Ok(())
}

