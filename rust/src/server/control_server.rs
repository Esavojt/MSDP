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

pub trait TCPServerTrait {
    fn get_socket(&self) -> &TcpListener;

    fn get_entries(&self) -> &Arc<Mutex<Vec<Entry>>>;

    fn handle_connections(&self) -> std::io::Result<()> {
        println!("Control server is listening on {}", self.get_socket().local_addr()?);
        for stream in self.get_socket().incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    let state = self.handle_client(stream);
                    if let Err(e) = state {
                        if e.kind() == std::io::ErrorKind::BrokenPipe || e.kind() == std::io::ErrorKind::ConnectionReset{
                            println!("Client disconnected: {}", e);
                        } else {
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, stream: std::net::TcpStream) -> std::io::Result<()>;
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
}

impl TCPServerTrait for ControlServer {
    fn get_socket(&self) -> &TcpListener {
        &self.socket
    }
    fn get_entries(&self) -> &Arc<Mutex<Vec<Entry>>> {
       &self.entries
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
