use std::{collections::HashMap, io::{BufRead, BufReader, BufWriter, Write}, net::TcpListener, os::unix::thread, sync::{Arc, Mutex}, thread::{sleep, spawn}, time::Duration, vec};

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
        log::info!("Got {} endpoint groups", entries.len());
        
        let mut buff: Vec<u8> = Vec::new();

        buff.extend_from_slice(b"# HELP msdp_entry_uptime Uptime of a server\n");
        buff.extend_from_slice(b"# TYPE msdp_entry_uptime counter\n");
        buff.extend_from_slice(b"# HELP msdp_entry_load Uptime of a server\n");
        buff.extend_from_slice(b"# TYPE msdp_entry_load gauge\n");
        for (name, entries) in &entries {
            for entry in entries {
                buff.extend_from_slice(
                    format!("msdp_entry_uptime{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                );
                buff.extend_from_slice(entry.uptime.to_string().as_bytes());
                buff.extend_from_slice(b"\n");

                buff.extend_from_slice(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"5\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                );
                buff.extend_from_slice(entry.load[0].to_string().as_bytes());
                buff.extend_from_slice(b"\n");

                buff.extend_from_slice(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"10\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                );
                buff.extend_from_slice(entry.load[1].to_string().as_bytes());
                buff.extend_from_slice(b"\n");

                buff.extend_from_slice(
                    format!("msdp_entry_load{{proxy_server_name=\"{}\", address=\"{}\", system_name=\"{}\", type=\"15\"}} ", name, entry.address, entry.system_name)
                    .as_bytes()
                );
                buff.extend_from_slice(entry.load[2].to_string().as_bytes());
                buff.extend_from_slice(b"\n");
            }
        }

        log::info!("Built response body of {} bytes", buff.len());
        
        let mut response = Vec::new();
        response.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
        response.extend_from_slice(b"Content-Type: text/plain; version=0.0.4\r\n");
        response.extend_from_slice(b"Connection: close\r\n");
        response.extend_from_slice(format!("Content-Length: {}\r\n\r\n", buff.len()).as_bytes());
        response.extend_from_slice(&buff);

        log::info!("Total response size: {} bytes", response.len());
        
        match stream.write_all(&response) {
            Ok(_) => log::info!("Successfully wrote response"),
            Err(e) => {
                log::error!("Failed to write response: {}", e);
                return Err(e);
            }
        }
        
        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Both)?;
        log::info!("Successfully handled client");
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