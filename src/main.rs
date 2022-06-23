use weathr::backend::WeatherList;
use weathr::display::WeatherForecast;

use std::error::Error;

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

    // Use environment variable as source of api key via export WEATHER_API_KEY="mykey"
    dotenv().ok();

//    let api_key: String = env::var("WEATHER_API_KEY").expect("export WEATHER_API_KEY=apikey not set");
//    let url = format!("https://api.openweathermap.org/data/2.5/forecast?q=Many&nbsp;Farms&appid={}&units=imperial", api_key);

//    dbg!(url);

//    let response = ureq::get(&url).call().unwrap().into_string().unwrap();
//    dbg!(response);

    let response = weathr::mock_json::JSON_RESPONSE;

    let data: WeatherList = WeatherList::parse(&response)?;
    let forecast: WeatherForecast = data.into();

//    dbg!(forecast);

    println!("{}", forecast);

    Ok(())
}

