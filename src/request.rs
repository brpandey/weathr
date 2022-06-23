use url::{Url, ParseError};
use ureq;


#[derive(Debug)]
pub enum Endpoint {
    CurrentWeather,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::CurrentWeather => String::from("forecast")
        }
    }
}

#[derive(Debug)]
pub enum Location {
    City(String),
    //    CityCountry(String, String),
    //    CityStateCountry(State, String, String),
}

impl Default for Location {
    fn default() -> Self { Location::City(String::from("London")) }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        match self {
            Self::City(ref name) => name.clone()
        }
    }
}

#[derive(Debug)]
pub enum Units {
    Metric,
    Imperial,
}

impl Default for Units {
    fn default() -> Self { Units::Imperial }
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            Self::Metric => String::from("metric"),
            Self::Imperial => String::from("imperial"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Request error")]
    Request(#[from] ureq::Error),
    #[error("Unable to transform response to string")]
    ConvertResponse(#[from] std::io::Error),
    #[error("Parsing URL error")]
    ParseURL(#[from] url::ParseError)
}

#[derive(Debug)]
pub struct WeatherApi {
    api_key: String,
    endpoint: Endpoint,
    location: Location,
    units: Units,
    debug: bool,
}

const BASE_API_URL: &str = "https://api.openweathermap.org/data/2.5";

impl WeatherApi {
    pub fn new(api_key: String, location: Location, units: Units) -> WeatherApi {
        WeatherApi {
            api_key,
            endpoint: Endpoint::CurrentWeather,
            location,
            units,
            debug: false,
        }
    }

    fn url_construct(&self) -> Result<String, ApiError> {
        let mut url = Url::parse(BASE_API_URL)?;

        println!("WeatherApi is {:?}", &self);

        let endpoint = &self.endpoint.to_string();
        url.path_segments_mut().unwrap().push(endpoint);

        url.query_pairs_mut()
            .append_pair("q", &self.location.to_string())
            .append_pair("appid", &self.api_key.to_string())
            .append_pair("units", &self.units.to_string());

        Ok(url.to_string())
    }

    pub fn request(&self) -> Result<String, ApiError> {
        let url = self.url_construct()?;
        let response = ureq::get(&url).call().unwrap().into_string().unwrap();

        Ok(response)
    }
}
