use std::fmt;
use std::collections::BTreeMap;

use colored::Colorize;
use tabular::{Table, Row};

use crate::backend::{WeatherList, DayKey, City};


/*
Flattened Structs and partially normalized
 */

#[allow(dead_code)]
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

// Final resultant top-level Weather struct to be used for output display
#[derive(Debug)]
pub struct WeatherForecast {
    location: City,
    days: BTreeMap<DayKey, Vec<WeatherSection>>,
}


impl WeatherForecast {
    pub(crate) fn new(location: City, days: BTreeMap<DayKey, Vec<WeatherSection>>) -> Self {
        WeatherForecast {
            location,
            days
        }
    }
}


// Implement `Display` for `WeatherForecast` using tabular
impl fmt::Display for WeatherForecast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print each days table
        //                             1     2    3      4      5     6       7         8
        let mut table = Table::new("{:>}    {:<}  {:<}   {:>}   {:>}  {:>}    {:>}   {:<}");

        let loc = format!("\n{}", &self.location.to_string().cyan().bold());
        table.add_heading(loc);

        for (k,v) in self.days.iter() {
            table.add_heading(format!("\n{}", &k.to_string().italic().purple()));
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

    // Implicit override of fmt::Display since it accepts table
    pub(crate) fn display(&self, table: &mut Table) {
        let day_hour = format!("{}-{:02}", &self.day_of_week, &self.hour).bold();
        let temp = format!("{:.2}", &self.temp).yellow();
        let feels = format!("{:.2}", &self.feels_like).bright_yellow().bold();
        let hum = format!("{}", &self.humidity).green();
        let wspeed = format!("{:.2}", &self.wind_speed).blue().on_bright_blue();
        let wdeg = format!("{:03}", &self.wind_deg).blue().on_bright_blue();
        let rainfall_3h: f32 = self.rain_three_hour.unwrap_or_default();
        let rainfall = format!("{:.2}", &rainfall_3h).blue().italic().on_bright_white();
        let desc = format!("{}", &self.description).bright_green().bold().italic();

        table.add_row(Row::new()
                      .with_cell(day_hour) // 1
                      .with_cell(temp) // 2
                      .with_cell(feels)  // 3
                      .with_cell(hum) // 4
                      .with_cell(wspeed) // 5
                      .with_cell(wdeg) // 6
                      .with_cell(rainfall) // 7
                      .with_cell(desc) // 8
        );
    }
}
