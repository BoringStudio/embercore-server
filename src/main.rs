fn main() {
    if let Err(e) = embercore_server::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
