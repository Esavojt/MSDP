use std::collections::HashMap;
use std::env;
use std::{io::Read, net::TcpStream};

use config::Map;
use rust::structs::entry::Entry;

fn main() -> std::io::Result<()> {
    let mut file_loc = String::from("client.toml");
    let mut args = env::args();

    if args.len() > 1 {
        // Set the address from the command line argument
        file_loc = args.nth(1).unwrap();
    }

    let conf = config::Config::builder()
        .add_source(config::File::with_name(&file_loc))
        .build()
        .unwrap();

    println!("Reading config file: {}", file_loc);
    let endpoints = conf.try_deserialize::<HashMap<String,Map<String,String>>>().unwrap();

    for (key, value) in &endpoints {
        let ip =  value.get("ip").expect("IP not found");
        let port = value.get("port").expect("Port not found");
        let addr = format!("{}:{}", ip, port);
        let name = value.get("name").unwrap_or(key);

        let data = get_data_from_socket(&addr)?;
        //println!("{}", data);

        let entries: Vec<Entry> = serde_json::from_str(&data)?;

        println!("List of neighbors:");
        println!("----- {}: {} -----", name, addr);
        let output = format_entries(&entries);
        println!("{}", output);
    }

    Ok(())
}

fn get_data_from_socket(addr: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(addr)?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}

fn format_entries(entries: &Vec<Entry>) -> String {
    let mut output = String::new();

    for entry in entries {
        output += &entry.format();
        output += "\n";
    }

    output
}
