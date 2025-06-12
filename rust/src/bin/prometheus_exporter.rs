use std::collections::HashMap;
use std::env;

use log::warn;
use rust::client::client::Client;
use rust::prometheus_exporter::exporter::ExportServer;
use rust::server::control_server::TCPServerTrait;

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
    let endpoints: HashMap<String, HashMap<String, String>> = conf
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .unwrap();

    let server  = ExportServer::new("0.0.0.0:10000", endpoints)?;

    println!("Handling new connections...");
    server.handle_connections()
}
