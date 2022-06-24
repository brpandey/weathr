use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct WeatherConfig {
    location: String,
    units: Option<String>,
    debug: bool,
}

impl WeatherConfig {
    fn new(location: String, units: Option<String>, debug: bool) -> Self {
        WeatherConfig {
            location,
            units,
            debug,
        }
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn location(&self) -> &str {
        self.location.as_ref()
    }

    pub fn units(&self) -> Option<&str> {
        self.units.as_ref().map(|s| s.as_ref())
    }

    pub fn load() -> Result<WeatherConfig, Box<dyn Error>> {
        let matches = App::new("weathr")
            .version("0.1.0")
            .author("Bibek Pandey")
            .about("Simple CLI Weather App")
            .arg(
                Arg::with_name("location")
                    .takes_value(true)
                    .required(true)
                    .help("City Location")
                    .short('l')
                    .long("location"),
            )
            .arg(
                Arg::with_name("debug")
                    .takes_value(false)
                    .help("Enable debug")
                    .short('d')
                    .long("debug"),
            )
            .arg(
                Arg::with_name("units")
                    .takes_value(true)
                    .possible_values(&["imperial", "metric"])
                    .help("Specify units type")
                    .short('u')
                    .long("units"),
            )
            .arg(
                Arg::with_name("output")
                    .takes_value(true)
                    .possible_values(&["tabular"])
                    .help("Display output type")
                    .short('o')
                    .long("output"),
            )
            .get_matches();

        let config = WeatherConfig::new(
            matches.value_of("location").unwrap().to_string(),
            matches.value_of("units").map(|s| s.to_string()),
            matches.is_present("debug"),
        );

        Ok(config)
    }
}
