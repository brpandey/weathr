extern crate chrono;

use std::error::Error;

use serde::Deserialize;
use chrono::prelude::DateTime;
use chrono::{Utc};
use std::time::{UNIX_EPOCH, Duration};

use crate::display::{WeatherForecast, WeatherSection};

/*
dt: 1655586000
main: {
temp: 92.41,
feels_like: 90.88,
humidity: 30
   },
   weather: {
     main: "Rain",
     description: "light rain",
     
   },
   wind: {
     speed: 21.43,
     deg: 236
   },
   rain: {
     3h: 0.22
   },
*/


/*
JSON "Strong-typed" parse into structs
*/

#[derive(Deserialize, Debug)]
pub struct WeatherList {
    cod: String,
    list: Vec<WeatherData>,
    city: City,
}

impl WeatherList {

    #[inline]
    pub fn parse(response: &str) -> Self {  // Result<WeatherList, Box<dyn Error>> {

        let wdata = serde_json::from_str(&response).unwrap();

        wdata
    }

    pub(crate) fn transform(&self) -> WeatherForecast {
        let list: Vec<WeatherSection> = self.list.iter().map(WeatherData::transform).collect();
        WeatherForecast::new(self.city.to_string(), list)
    }
}


#[derive(Deserialize, Debug)]
struct WeatherData {
    #[serde(rename = "dt")] 
    datetime: u64,
    main: Temperature,
    weather: Vec<Description>,
    wind: Wind,
    rain: Option<Rain>,
}

impl WeatherData {
    pub fn transform(&self) -> WeatherSection {

        // datetime
        let system_time = UNIX_EPOCH + Duration::from_secs(self.datetime);
        let datetime = DateTime::<Utc>::from(system_time);

        // Formats the combined date and time with the specified format string.
        let timestamp: String = datetime.format("%Y-%m-%d %H:%M").to_string();

        WeatherSection::new(
            timestamp.clone(),
            self.main.temp,
            self.main.feels_like,
            self.main.humidity,
            self.weather[0].description.clone(),
            self.wind.speed,
            self.wind.deg,
            self.rain.as_ref().map(|r| r.three_hour),
        )
    }
}

#[derive(Deserialize, Debug)]
struct Temperature {
    temp: f32,
    feels_like: f32,
    humidity: u16,
}

#[derive(Deserialize, Debug)]
struct Description {
    main: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: u16,
}

#[derive(Deserialize, Debug)]
struct Rain {
    #[serde(rename = "3h")] 
    three_hour: f32,
}


#[derive(Deserialize, Debug)]
struct City {
    name: String,
    country: String,
}

impl ToString for City {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.country)
    }
}


