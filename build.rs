use std::{ fs, error, env};
use std::path::Path;

use dotenv::*;
use path_slash::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_dir(protocol_dir: &str) -> Result<Vec<String>> {
    let current_dir = Path::new(protocol_dir);

    let result = match fs::read_dir(current_dir) {
        Ok(dirs) => dirs,
        Err(e) => return Err(Box::new(e))
    };

    let result = result
        .filter_map(|entry| {
            let path = entry.unwrap().path();

            if path.is_file() && path.extension().unwrap_or_default() == "proto" {
                Some(path.to_slash().unwrap())
            } else {
                None
            }
        })
        .collect();

    Ok(result)
}

fn main() {
    dotenv().ok();

    let protocol_dir = match env::var("PROTOCOL_DIR") {
        Ok(dir) => dir,
        Err(_) => panic!("PROTOCOL_DIR must be set")
    };

    let files = match read_dir(&protocol_dir) {
        Ok(files) => files,
        Err(e) => panic!("Error: {}", e.to_string())
    };

    println!("ASD");

    let compiled = prost_build::Config::new()
        .out_dir("./src/protocol")
        .compile_protos(files.as_slice(), &[protocol_dir]);

    if let Err(e) = compiled {
        eprintln!("Error: {}", e.to_string());
    }
}
