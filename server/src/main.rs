use std::io::Write;

use clap::{App, Arg};

use embercore_server_lib::config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let log_filters = std::env::var("RUST_LOG").unwrap_or_default();

    env_logger::Builder::new()
        .parse_filters(&log_filters)
        .format(|formatter, record| {
            writeln!(
                formatter,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    embercore_server_lib::run(config).await
}
