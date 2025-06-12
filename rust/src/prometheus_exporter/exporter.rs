use std::{collections::HashMap, io::Write, net::TcpListener, sync::{Arc, Mutex}};

use crate::{client::client::Client, server::control_server::TCPServerTrait, structs::entry::Entry};

pub struct ExportServer {
    socket: std::net::TcpListener,
    endpoints: HashMap<String, HashMap<String, String>>
}

impl TCPServerTrait for ExportServer {
    fn get_socket(&self) -> &std::net::TcpListener {
        &self.socket
    }

    fn get_entries(&self) -> &Arc<Mutex<Vec<Entry>>> {
        todo!()
    }

    fn handle_client(&self, mut stream: std::net::TcpStream) -> std::io::Result<()> {
        log::info!("Handling new connection...");
        let entries = self.get_entries_from_endpoints();
        stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
        stream.write_all(b"Content-Type: text/plain; version=0.0.4\r\n")?;
        stream.write_all(b"Connection:close\r\n\r\n")?;
        stream.write_all(b"# HELP msdp_entry_uptime Uptime of a server\n")?;
        stream.write_all(b"# TYPE msdp_entry_uptime counter\n")?;
        stream.write_all(b"# HELP msdp_entry_load Uptime of a server\n")?;
        stream.write_all(b"# TYPE msdp_entry_load gauge\n")?;
        for (name, entries) in &entries {
            for entry in entries {
                stream.write_all(
                    format!("msdp_entry_uptime{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                )?;
                stream.write_all(entry.uptime.to_string().as_bytes())?;
                stream.write_all(b"\n")?;

                stream.write_all(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"5\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                )?;
                stream.write_all(entry.load[0].to_string().as_bytes())?;
                stream.write_all(b"\n")?;

                stream.write_all(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"10\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                )?;
                stream.write_all(entry.load[1].to_string().as_bytes())?;
                stream.write_all(b"\n")?;

                stream.write_all(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"15\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                )?;
                stream.write_all(entry.load[2].to_string().as_bytes())?;
                stream.write_all(b"\n")?;
            }
        }
        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
        
    }
    fn handle_connections(&self) -> std::io::Result<()> {
        for stream in self.get_socket().incoming() {
            let stream = stream?;
            self.handle_client(stream)?;
        }
        Ok(())
    }
}

impl ExportServer {
    pub fn new(addr: &str, endpoints: HashMap<String, HashMap<String, String>>) -> std::io::Result<Self> {
        let socket = TcpListener::bind(addr)?;
        Ok(ExportServer {
            socket,
            endpoints
        })
    }
    fn get_entries_from_endpoints(&self) -> HashMap<&str, Vec<Entry>>{
        let mut map: HashMap<&str, Vec<Entry>> = HashMap::new();
        for (key, value) in &self.endpoints {
            let ip = value.get("ip").expect("IP not found");
            let port = value.get("port").expect("Port not found");
            let addr = format!("{}:{}", ip, port);
            let name = value.get("name").unwrap_or(key);

            match Client::connect(&addr) {
                Ok(entries) => {
                    map.insert(name, entries);
                }
                Err(error) => {
                    log::error!("Error reading from {addr}: {error}");
                }
            };    
        }
        map
    }
}