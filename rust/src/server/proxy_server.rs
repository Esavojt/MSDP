use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::structs::entry::Entry;
use crate::server::control_server::TCPServerTrait;

pub struct ProxyServer {
    socket: TcpListener,
    addr: String,
    entries: Arc<Mutex<Vec<Entry>>>,
}

impl TCPServerTrait for ProxyServer {
    fn get_socket(&self) -> &TcpListener {
        &self.socket
    }

    fn get_entries(&self) -> &Arc<Mutex<Vec<Entry>>> {
        &self.entries
    }

    fn handle_connections(&self) -> std::io::Result<()> {
        println!("Proxy server is listening on {}", self.get_socket().local_addr()?);
        for stream in self.get_socket().incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection to proxy: {}", stream.peer_addr()?);
                    self.handle_client(stream)?;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: std::net::TcpStream) -> std::io::Result<()> {
        let json = {
            let entries = self.get_entries().lock().unwrap();
            serde_json::to_string(&*entries)?
        };

        // Send the JSON data to the client
        stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
        stream.write_all(b"Content-Type: application/json\r\n")?;
        stream.write_all(b"Content-Length: ")?;
        stream.write_all(json.len().to_string().as_bytes())?;
        stream.write_all(b"\r\n")?;
        stream.write_all(b"Connection: close\r\n")?;
        stream.write_all(b"Access-Control-Allow-Origin: *\r\n")?;
        stream.write_all(b"\r\n")?;

        stream.write_all(json.as_bytes())?;
        //stream.write_all(b"\r\n")?;


        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
    
}

impl ProxyServer {
    pub fn new(addr: String, entries: Arc<Mutex<Vec<Entry>>>) -> std::io::Result<Self> {
        let socket = TcpListener::bind(&addr)?;
        Ok(ProxyServer {
            socket,
            addr,
            entries,
        })
    }
}