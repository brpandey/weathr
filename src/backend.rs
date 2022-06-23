extern crate chrono;


use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{UNIX_EPOCH, Duration};

use serde::Deserialize;
use chrono::prelude::DateTime;
use chrono::{Utc};
use chrono::TimeZone;
use chrono::FixedOffset;

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
    pub fn parse(response: &str) -> Result<WeatherList, Box<dyn Error>> {
        let data = serde_json::from_str(&response);
        data.map_err(From::from)
    }

    // transforms raw weather list into weather forecast
    pub(crate) fn transform(&self) -> WeatherForecast {

        let mut map: BTreeMap<DayKey, Vec<WeatherSection>> = BTreeMap::new();
        let exclude_hours: Vec<u8> = vec![0, 3];
        let exclusion: HashSet<u8> = exclude_hours.into_iter().collect();  // merge only those hours not on the exclusion list

        let list: Vec<(DayKey, u8, WeatherSection)> = self.list.iter().map(WeatherData::transform).collect();

        map = list.into_iter().fold(map, |mut acc, (day, hour, ws)| {
            if !exclusion.contains(&hour) {
                acc.entry(day)
                    .and_modify(|v| v.push(ws.clone()))
                    .or_insert(vec![ws.clone()]);
            }

            acc
        });

        WeatherForecast::new(self.city.clone(), map)
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct DayKey {
    month: u8,
    day: u8,
}

impl ToString for DayKey {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}", self.month, self.day)
    }
}

impl FromStr for DayKey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split('-').collect();
        let month = tokens[0].parse::<u8>()?;
        let day = tokens[1].parse::<u8>()?;

        Ok(DayKey { month, day })
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
    pub fn transform(&self) -> (DayKey, u8, WeatherSection) {

        // datetime
        let system_time = UNIX_EPOCH + Duration::from_secs(self.datetime);
        let datetime = DateTime::<Utc>::from(system_time);

        // Formats the combined date and time with the specified format string.
        let timestamp: String = datetime.format("%Y-%m-%d %H:%M").to_string();
        let day_of_week: String = datetime.format("%a").to_string();
        let day_temp: String = datetime.format("%m-%d").to_string();

        let day_key = DayKey::from_str(&day_temp).unwrap();
        let hour_int: u8 = datetime.format("%H").to_string().parse().unwrap();

        (day_key, hour_int, WeatherSection::new(
            day_of_week,
            hour_int,
            timestamp,
            self.main.temp,
            self.main.feels_like,
            self.main.humidity,
            self.weather[0].description.clone(),
            self.wind.speed,
            self.wind.deg,
            self.rain.as_ref().map(|r| r.three_hour),
        ))
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


#[derive(Clone, Deserialize, Debug)]
pub struct City {
    name: String,
    country: String,
    coord: Coord,
    sunrise: u64,
    sunset: u64,
    timezone: i32,
}


#[derive(Copy, Clone, Deserialize, Debug)]
struct Coord {
    lat: f32,
    lon: f32,
}


impl ToString for City {
    fn to_string(&self) -> String {
        // datetime
        let sunrise: String = datetime(self.sunrise, self.timezone);
        let sunset: String = datetime(self.sunset, self.timezone);

        format!("{} {} [{}, {}]\nSunrise {}  Sunset {}", self.name, self.country, self.coord.lat, self.coord.lon, sunrise, sunset)
    }
}

pub fn datetime(value: u64, offset: i32) -> String {
    let system_time = UNIX_EPOCH + Duration::from_secs(value);
    let d = DateTime::<Utc>::from(system_time);

    let tz: FixedOffset = if offset < 0 { FixedOffset::east(offset) } else { FixedOffset::west(offset) };
    let dtz = d.with_timezone(&tz);
    dtz.format("%m-%d %H:%M").to_string()
}
