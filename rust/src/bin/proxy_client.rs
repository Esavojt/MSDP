use std::collections::HashMap;
use std::env;

use config::Map;
use log::warn;
use rust::client::client::Client;
use rust::structs::entry::Entry;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut args = env::args();
    
    let file_loc: String = match args.nth(1) {
        Some(arg) => arg,
        None => {
            warn!("No config file provided. Using default: config.toml");
            String::from("config.toml")
        }
    };

    let conf = config::Config::builder()
        .add_source(config::File::with_name(&file_loc))
        .build()
        .expect("Failed to load config file");

    println!("Reading config file: {}", file_loc);
    let endpoints = conf.try_deserialize::<HashMap<String,Map<String,String>>>().unwrap();

    for (key, value) in &endpoints {
        let ip =  value.get("ip").expect("IP not found");
        let port = value.get("port").expect("Port not found");
        let addr = format!("{}:{}", ip, port);
        let name = value.get("name").unwrap_or(key);

        let entries: Vec<Entry> = Client::connect(&addr)?;

        println!("List of neighbors:");
        println!("=========== {}: {} ===========", name, addr);
        let output = Client::format_entries(&entries);
        println!("{}", output);
    }

    Ok(())
}
