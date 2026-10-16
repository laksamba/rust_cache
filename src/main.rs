use std::eprintln;


mod command;
mod server;
mod store;

fn main() {
    if let Err(error) = server::run("127.0.0.1:6379") {
        eprintln!("Server error: {error}");
        std::process::exit(1);
    }
}
