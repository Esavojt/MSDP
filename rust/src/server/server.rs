use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use log::{debug, error, info};

use std::time::Duration;
use uuid::Uuid;

use std::time::SystemTime;

use crate::structs::entry::Entry;
use crate::structs::message::MessageV1;

const BUFFER_SIZE: usize = 2048;

pub struct MSDPServer {
    pub unique_id: Uuid,
    pub multicast_group: [u8; 4],
    pub port: u16,
    pub socket: UdpSocket,
    pub keepalive_timer: u16,
    pub entries: Arc<Mutex<Vec<Entry>>>, // Use Arc<Mutex<>>
}

impl MSDPServer {
    pub fn new(
        multicast_group: [u8; 4],
        port: u16,
        keepalive_timer: u16,
        entries: Arc<Mutex<Vec<Entry>>>, // Accept Arc<Mutex<>>
    ) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", &port))?;
        let [a, b, c, d] = multicast_group;
        socket.join_multicast_v4(&Ipv4Addr::new(a, b, c, d), &Ipv4Addr::from_bits(0))?;
        socket.set_multicast_ttl_v4(1)?;
        socket.set_read_timeout(Some(Duration::from_secs((keepalive_timer / 2) as u64)))?;

        Ok(MSDPServer {
            multicast_group,
            port,
            socket,
            keepalive_timer,
            unique_id: Uuid::new_v4(),
            entries, // Store the shared entries
        })
    }

    fn send_message(&self) -> Result<(), std::io::Error> {
        let uptime = uptime_lib::get()
            .unwrap_or_else(|_| Duration::new(0, 0));

        let load = sys_info::loadavg().unwrap_or(sys_info::LoadAvg {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
        });

        let message = MessageV1::new(
            self.unique_id,
            sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string()),
            sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string()),
            self.keepalive_timer,
            uptime.as_secs() as u32,
            [load.one as f32, load.five as f32, load.fifteen as f32],
        )
        .to_bytes();

        let [a, b, c, d] = self.multicast_group;
        self.socket
            .send_to(&message, (Ipv4Addr::new(a, b, c, d), self.port))?;
        Ok(())
    }

    fn receive_message(&self) -> Result<Result<(MessageV1, SocketAddr), ()>, std::io::Error> {
        let mut buf = [0; BUFFER_SIZE];

        // Attempt to receive data from the socket
        let (size, addr) = match self.socket.recv_from(&mut buf) {
            Ok(result) => result,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                debug!("No data received, continuing...");
                return Ok(Err(()));
            }
            Err(e) => {
                error!("Error receiving data: {}", e);
                return Err(e);
            }
        };

        // Attempt to parse the received message
        match MessageV1::parse(&buf[..size]) {
            Ok(msg) if msg.unique_id == self.unique_id => {
                debug!("Received message from self, ignoring");
                Ok(Err(()))
            }
            Ok(msg) => {
                info!("Received a valid v{} message from {:?}", msg.version, addr);
                Ok(Ok((msg, addr)))
            }
            Err(_) => {
                error!("Failed to parse message from {:?}", addr);
                Ok(Err(()))
            }
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut last_send = std::time::Instant::now();
        loop {
            // Send a message every keepalive_timer seconds
            if last_send.elapsed() >= Duration::from_secs(self.keepalive_timer as u64) {
                self.send_message()?;
                last_send = std::time::Instant::now();
            }

            match self.receive_message() {
                Ok(Ok((message, address))) => {
                    // Process the received message
                    self.process_message(message, address);
                }
                Ok(Err(())) => {
                    // Ignore the message
                    debug!("Ignoring message");
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    return Err(e);
                }
            }
        }
    }

    fn process_message(&mut self, msg: MessageV1, addr: SocketAddr) {
        // Access the shared entries
        let mut entries = self.entries.lock()
            .expect("Failed to lock entries");

        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update or add entries
        let mut found = false;
        for entry in &mut *entries {
            if entry.unique_id == msg.unique_id {
                found = true;
                entry.last_seen = now;
                entry.uptime = msg.uptime;
                entry.load = msg.load;
                entry.keepalive_timer = msg.keepalive_timer;
                info!("Updated entry: {}", msg.system_name);
            }
        }

        if !found {
            info!("New entry: {}", msg.system_name);
            let entry = Entry::from_message(msg, addr);
            entries.push(entry);
        }

        // Retain only recent entries
        entries.retain(|entry| {
            entry.last_seen + (self.keepalive_timer as u64 * 2) >= now
        });
    }
}
