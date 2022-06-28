use std::str::FromStr;
use url::Url;
use ureq;

// Module serves to wrap functionality around placing
// a HTTP request.

// Currently only tailored to one web service

#[derive(Debug)]
pub enum Endpoint {
    CurrentWeather,
}

impl Endpoint {
    fn value(&self) -> &str {
        match self {
            Self::CurrentWeather => "forecast"
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

impl FromStr for Location {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::City(s.to_owned()))
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

impl FromStr for Units {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "metric" => Ok(Self::Metric),
            "imperial" => Ok(Self::Imperial),
            _ => Err(ApiError::BadParse("unknown str unit type"))
        }
    }
}

// Custom Error enum that provides relevant detail
// as an alternative over Box<dyn Error> which must
// use type downcasting to extract original error info

// error macro annotation simplifies implementation of
// Debug/Display traits required by the Error trait
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Request error")]
    Request(#[from] ureq::Error),
    #[error("Unable to transform response to string")]
    ConvertResponse(#[from] std::io::Error),
    #[error("Parsing URL error")]
    ParseURL(#[from] url::ParseError),
    #[error("Unsupported parse type {0}")]
    BadParse(&'static str),

}


// Defines data and functionality to place an HTTP GET request
// Extensible to add new endpoints, and other restful param types

#[derive(Debug)]
pub struct WeatherApi {
    api_key: String,
    endpoint: Endpoint,
    location: Location,
    units: Units,
//    debug: bool,
}

const BASE_API_URL: &str = "https://api.openweathermap.org/data/2.5";

impl WeatherApi {
    pub fn new(api_key: String, location: Location, units: Units) -> WeatherApi {
        WeatherApi {
            api_key,
            endpoint: Endpoint::CurrentWeather,
            location,
            units,
//            debug: false,
        }
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_ref()
    }

    pub fn location(&self) -> &str {
        match self.location {
            Location::City(ref name) => name.as_ref()
        }
    }

    pub fn units(&self) -> &str {
        match self.units {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    }

    // Generate WeatherApi given api_key and user supplied cli args
    pub fn load(api_key: String, location: &str, units_opt: Option<&str>) -> Result<WeatherApi, ApiError> {
        let loc = Location::from_str(location)?;

        let units = if let Some(u) = units_opt {
            Units::from_str(u).unwrap_or(Units::default())
        } else {
            Units::default()
        };

        Ok(WeatherApi::new(api_key, loc, units))
    }

    // Generate final url given api parameters
    fn url_construct(&self) -> Result<String, ApiError> {
        let mut url = Url::parse(BASE_API_URL)?;
        let endpoint = self.endpoint.value();

        url.path_segments_mut().unwrap().push(endpoint);

        url.query_pairs_mut()
            .append_pair("q", self.location())
            .append_pair("appid", self.api_key())
            .append_pair("units", self.units());

        Ok(url.to_string())
    }

    // Places HTTP get (blocking) call using ureq
    pub fn request(&self) -> Result<String, ApiError> {
        let url = self.url_construct()?;
        let response = ureq::get(&url).call()?.into_string()?;

        Ok(response)
    }
}
