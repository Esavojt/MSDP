use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use log::info;

use crate::server::control_server::TCPServerTrait;
use crate::structs::entry::Entry;

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

    fn handle_client(&self, mut stream: std::net::TcpStream) -> std::io::Result<()> {
        let json = {
            let entries = self.get_entries()
                .lock()
                .expect("Failed to lock entries");
            serde_json::to_string(&*entries)?
        };

        info!("Sending JSON data to proxy client");

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
