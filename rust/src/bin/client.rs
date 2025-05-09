use std::env;

use log::warn;
use rust::client::client::Client;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut args = env::args();

    let addr: String = match args.nth(1) {
        Some(arg) => arg,
        None => {
            warn!("No config file provided. Using default: config.toml");
            String::from("localhost:10001")
        }
    };
    let entries = Client::connect(&addr)?;
    //println!("{}", data);

    println!("List of neighbors:");

    let output = Client::format_entries(&entries);
    println!("{}", output);

    Ok(())
}
