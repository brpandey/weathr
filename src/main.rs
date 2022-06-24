﻿use std::env;
use std::error::Error;


use dotenv::dotenv;

use weathr::config::WeatherConfig;
use weathr::request::WeatherApi;
use weathr::backend::WeatherList;
use weathr::display::WeatherForecast;

fn main() -> Result<(), Box<dyn Error>> {

    // Use environment variable as source of api key via export WEATHER_API_KEY="mykey"
    dotenv().ok();

    let api_key: String = env::var("WEATHER_API_KEY").expect("export WEATHER_API_KEY= not set");

    let config = WeatherConfig::load()?;
    let api = WeatherApi::load(api_key, config.location(), config.units())?;

    let response = api.request()?;

    let data: WeatherList = WeatherList::parse(&response)?;
    let forecast: WeatherForecast = data.into();

    println!("{}", forecast);

    Ok(())
}

