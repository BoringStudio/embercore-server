use clap::{
    App,
    Arg,
};

use embercore_server_lib::config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    embercore::utils::init_logger();

    let matches = App::new("embercore-server")
        .version("1.0")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config = {
        let path = matches.value_of("config").unwrap();
        Config::new(path)?
    };

    embercore_server_lib::run(config).await
}
