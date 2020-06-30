use clap::{App, Arg, ArgMatches};

use anyhow::Result;
use config::ConfigError;
use embercore_server_lib::config::*;

#[tokio::main]
async fn main() -> Result<()> {
    embercore::utils::init_logger();
    let app = init_cli();
    let config = init_config(app.get_matches())?;

    embercore_server_lib::run(config).await
}

fn init_cli() -> App {
    App::new("embercore-server").version("1.0").arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
            .required(true),
    )
}

fn init_config(args: ArgMatches) -> Result<Config, ConfigError> {
    let path = matches.value_of("config").unwrap();
    Config::new(path)
}
