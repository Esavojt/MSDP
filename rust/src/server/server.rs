use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use std::time::Duration;
use uuid::Uuid;

use std::time::SystemTime;

use crate::structs::entry::Entry;
use crate::structs::message::MessageV1;

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
        let uptime = uptime_lib::get().unwrap();
        let load = sys_info::loadavg().unwrap();

        let message = MessageV1::new(
            self.unique_id.clone(),
            sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string()),
            sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string()),
            60,
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
        let mut buf = [0; 2048];

        let data = self.socket.recv_from(&mut buf);

        if let Err(e) = data {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                return Ok(Err(()));
            } else {
                return Err(e);
            }
        }

        let (size, addr) = data.unwrap();

        let message = MessageV1::parse(&buf[..size]);

        if let Ok(msg) = message {
            if msg.unique_id == self.unique_id {
                //println!("Received message from self, ignoring");
                return Ok(Err(()));
            }
            Ok(Ok((msg, addr)))
        } else {
            println!("Failed to parse message from {:?}", addr);
            Ok(Err(()))
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut last_send = std::time::Instant::now();
        loop {
            if last_send.elapsed() >= Duration::from_secs(self.keepalive_timer as u64) {
                self.send_message()?;
                last_send = std::time::Instant::now();
            }

            let message = self.receive_message()?;
            if message.is_err() {
                continue;
            }

            let (msg, addr) = message.unwrap();

            // Access the shared entries
            let mut entries = self.entries.lock().unwrap();

            // Update or add entries
            let mut found = false;
            for entry in &mut *entries {
                if entry.unique_id == msg.unique_id {
                    found = true;
                    entry.last_seen = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    entry.uptime = msg.uptime;
                    entry.load = msg.load;
                    entry.keepalive_timer = msg.keepalive_timer;
                }
            }

            if !found {
                let entry = Entry::from_message(msg, addr);
                entries.push(entry);
            }

            // Retain only recent entries
            entries.retain(|entry| {
                entry.last_seen + (self.keepalive_timer as u64 * 2)
                    >= SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
            });
        }
    }
}
