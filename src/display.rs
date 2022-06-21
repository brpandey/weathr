use crate::backend::WeatherList;

/*
Display Structs
 */

#[derive(Debug)]
pub struct WeatherForecast {
    location: String,
    sections: Vec<WeatherSection>,
}


impl WeatherForecast {
    pub(crate) fn new(location: String, sections: Vec<WeatherSection>) -> Self {
        WeatherForecast {
            location,
            sections
        }
    }
}

impl From<WeatherList> for WeatherForecast {
    fn from(data: WeatherList) -> Self {
        data.transform()
    }
}


#[derive(Debug)]
pub(crate) struct WeatherSection {
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
            datetime, temp, feels_like, humidity,
            description, wind_speed, wind_deg, rain_three_hour
        }
    }
}
