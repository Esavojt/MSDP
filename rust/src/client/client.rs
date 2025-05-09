use std::{io::Read, net::TcpStream};

use log::error;

use crate::structs::entry::Entry;

pub struct Client {}

impl Client {
    pub fn connect(addr: &str) -> std::io::Result<Vec<Entry>> {
        let data = Client::get_data_from_socket(addr)?;

        let json = serde_json::from_str(&data).map_err(|e| {
            error!("Failed to parse JSON: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;

        Ok(json)
    }

    fn get_data_from_socket(addr: &str) -> std::io::Result<String> {
        let mut stream = TcpStream::connect(addr)?;
        let mut response: String = String::new();
        stream.read_to_string(&mut response)?;

        Ok(response)
    }

    pub fn format_entries(entries: &Vec<Entry>) -> String {
        entries
            .iter()
            .map(|entry| entry.format())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
