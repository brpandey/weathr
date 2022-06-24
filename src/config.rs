use clap::{App, Arg};

// Wrapper around correctly matched user supplied args

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

    #[inline]
    pub fn debug(&self) -> bool {
        self.debug
    }

    #[inline]
    pub fn location(&self) -> &str {
        self.location.as_ref()
    }

    #[inline]
    pub fn units(&self) -> Option<&str> {
        self.units.as_ref().map(|s| s.as_ref())
    }

    // Match the params expected with the params actually given using clap
    pub fn load() -> WeatherConfig {
        let matches = App::new("weathr")
            .version("0.1.0")
            .author("Bibek Pandey")
            .about("Simple CLI Weather App using OpenWeatherMap")
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

        // Unpack the args that have matched successfully and capture in Config
        WeatherConfig::new(
            matches.value_of("location").unwrap().to_string(),
            matches.value_of("units").map(|s| s.to_string()),
            matches.is_present("debug"),
        )
    }
}
