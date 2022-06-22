
use std::fmt;

use std::collections::BTreeMap;
use tabular::{Table, Row};
use crate::backend::{WeatherList, DayKey};

/*
Flattened Structs
 */

#[derive(Debug, Clone)]
pub(crate) struct WeatherSection {
    day_of_week: String,
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


#[derive(Debug)]
pub struct WeatherForecast {
    location: String,
    days: BTreeMap<DayKey, Vec<WeatherSection>>,
}


impl WeatherForecast {
    pub(crate) fn new(location: String, days: BTreeMap<DayKey, Vec<WeatherSection>>) -> Self {
        WeatherForecast {
            location,
            days
        }
    }
}


// Implement `Display` for `WeatherForecast`.
impl fmt::Display for WeatherForecast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Print each days table
        //                             1     2    3      4      5     6       7         8
        let mut table = Table::new("{:>}    {:<}  {:<}   {:>}   {:>}  {:>}    {:>}   {:<}");

        for (k,v) in self.days.iter() {

            table.add_heading(format!("\n{}", &k.to_string()));
            table.add_heading("day-hour  temp    feel   hum   wspd  wdeg   rain   desc");

            for section in v.iter() {
                section.display(&mut table)
            }
        }

        write!(f, "{}", table)
    }
}

impl From<WeatherList> for WeatherForecast {
    fn from(data: WeatherList) -> Self {
        data.transform()
    }
}


impl WeatherSection {
    pub(crate) fn new(
        day_of_week: String, hour: u8, datetime: String, temp: f32, feels_like: f32,
        humidity: u16, description: String, wind_speed: f32, wind_deg: u16,
        rain_three_hour: Option<f32>,
    ) -> Self {
        WeatherSection {
            day_of_week, hour, datetime, temp, feels_like, humidity,
            description, wind_speed, wind_deg, rain_three_hour
        }
    }

    pub(crate) fn display(&self, table: &mut Table) {
        let day_hour = format!("{}-{:02}", &self.day_of_week, &self.hour);
        let temp = format!("{:.2}", &self.temp);
        let feels = format!("{:.2}", &self.feels_like);
        let wspeed = format!("{:.2}", &self.wind_speed);
        let wdeg = format!("{:03}", &self.wind_deg);

        let rainfall_3h: f32 = self.rain_three_hour.unwrap_or_default();
        let rainfall = format!("{:.2}", &rainfall_3h);

        table.add_row(Row::new()
                      .with_cell(day_hour) // 1
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
