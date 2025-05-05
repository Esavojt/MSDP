use std::{
    io::Write,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use crate::structs::entry::Entry;

pub struct ControlServer {
    socket: TcpListener,
    addr: String,
    entries: Arc<Mutex<Vec<Entry>>>,
}

impl ControlServer {
    pub fn new(addr: String, entries: Arc<Mutex<Vec<Entry>>>) -> std::io::Result<Self> {
        let socket = TcpListener::bind(&addr)?;
        Ok(ControlServer {
            socket,
            addr,
            entries,
        })
    }

    pub fn handle_connections(&self) -> std::io::Result<()> {
        println!("Control server is listening on {}", self.addr);
        for stream in self.socket.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
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
            let entries = self.entries.lock().unwrap();
            serde_json::to_string(&*entries)?
        };

        stream.write_all(json.as_bytes())?;

        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}
