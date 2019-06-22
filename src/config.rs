use std::env;
use std::str::FromStr;

use dotenv;

pub struct Config {
    pub port: u16
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        dotenv::dotenv().ok();

        let port = find_var("PORT")
            .unwrap_or(10101);

        //TODO: parse other variables

        Ok(Config {
            port,
        })
    }
}

fn find_var<T: FromStr>(name: &str) -> Option<T> {
    env::var(name).ok()
        .and_then(|v| {
            v.parse().ok()
        })
}
