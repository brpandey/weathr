extern crate chrono;

use std::error::Error;
use std::collections::BTreeMap;

use serde::Deserialize;
use chrono::prelude::DateTime;
use chrono::{Utc};
use std::time::{UNIX_EPOCH, Duration};

use tabular::{Table, Row};

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

    pub(crate) fn transform(&self) -> WeatherForecast {

        let map: BTreeMap<String, Vec<WeatherSection>> = BTreeMap::new();

        let list: Vec<(String, WeatherSection)> = self.list.iter().map(WeatherData::transform).collect();

        let map = list.into_iter().fold(map, |mut acc, (day, ws)| {
            acc.entry(day).and_modify(|v| v.push(ws.clone())).or_insert(vec![ws.clone()]);
            acc
        });

        println!("{:?}", &map);

        WeatherForecast::new(self.city.to_string(), map)
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
    pub fn transform(&self) -> (String, WeatherSection) {

        // datetime
        let system_time = UNIX_EPOCH + Duration::from_secs(self.datetime);
        let datetime = DateTime::<Utc>::from(system_time);

        // Formats the combined date and time with the specified format string.
        let timestamp: String = datetime.format("%Y-%m-%d %H:%M").to_string();
        let day: String = datetime.format("%m-%d %a").to_string();
        let hour: u8 = datetime.format("%H").to_string().parse().unwrap();

        (day.clone(), WeatherSection::new(
            day.clone(),
            hour,
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


/*
Flattened Structs
 */

#[derive(Debug)]
pub struct WeatherForecast {
    location: String,
    days: BTreeMap<String, Vec<WeatherSection>>,
}


impl WeatherForecast {
    pub(crate) fn new(location: String, days: BTreeMap<String, Vec<WeatherSection>>) -> Self {
        WeatherForecast {
            location,
            days
        }
    }

    pub fn display(&self) {

        for (k,v) in self.days.iter() {
            println!("{}", &k);

            // Print each days table
            //                             1     2    3      4      5     6       7         8
            let mut table = Table::new("{:>}    {:<}  {:<}   {:>}   {:>}  {:>}    {:>}   {:<}");
            table.add_heading("hour  temp    feel   hum   wspd  wdeg   rain   desc");

            for section in v.iter() {
                section.display(&mut table)
            }

            let f =  format!("{}", table);
            println!("{}", f);

        }
    }
}

impl From<WeatherList> for WeatherForecast {
    fn from(data: WeatherList) -> Self {
        data.transform()
    }
}


#[derive(Debug, Clone)]
pub(crate) struct WeatherSection {
    day: String,
    hour: u8,
    datetime: String,
    temp: f32,
    feels_like: f32, 
    humidity: u16,
    description: String,
    wind_speed: f32,
    wind_deg: u16,
    rain_three_hour: Option<f32>,
}


impl WeatherSection {
    pub(crate) fn new(
        day: String,
        hour: u8,
        datetime: String,
        temp: f32,
        feels_like: f32,
        humidity: u16,
        description: String,
        wind_speed: f32,
        wind_deg: u16,
        rain_three_hour: Option<f32>,
    ) -> Self {
        WeatherSection {
            day, hour, datetime, temp, feels_like, humidity,
            description, wind_speed, wind_deg, rain_three_hour
        }
    }

    pub(crate) fn display(&self, table: &mut Table) {

        let hour = format!("{:02}", &self.hour);
        let temp = format!("{:.2}", &self.temp);
        let feels = format!("{:.2}", &self.feels_like);
        let wspeed = format!("{:.2}", &self.wind_speed);
        let wdeg = format!("{:03}", &self.wind_deg);

        let rainfall_3h: f32 = self.rain_three_hour.unwrap_or_default();
        let rainfall = format!("{:.2}", &rainfall_3h);

        //                             1     2    3      4      5     6       7         8
        // let mut table = Table::new("{:>}    {:>}  {:>}   {:>}   {:>}  {:>}    {:>}      {:<}");
        // table.add_heading("hour  temp   feels   hum  wspd   wdeg   rainfall  desc")
        table.add_row(Row::new()
                      .with_cell(hour) // 1
                      .with_cell(temp) // 2
                      .with_cell(feels)  // 3
                      .with_cell(self.humidity) // 4
                      .with_cell(wspeed) // 5
                      .with_cell(wdeg) // 6
                      .with_cell(rainfall) // 7
                      .with_cell(&self.description) // 8
        );
    }
}
