use std::{io::Read, net::TcpStream};

use rust::structs::entry::Entry;

fn main() -> std::io::Result<()> {
    let data = get_data_from_socket("localhost:10001")?;
    //println!("{}", data);

    let entries: Vec<Entry> = serde_json::from_str(&data)?;

    println!("List of neighbors:");

    let output = format_entries(&entries);
    println!("{}", output);

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
