use std::{
    net::SocketAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use log::error;
use serde::{Deserialize, Serialize};

use crate::format_time;

use super::message;

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub unique_id: uuid::Uuid,
    pub system_name: String,
    pub system_platform: String,
    pub system_version: String,
    pub keepalive_timer: u16,
    pub address: String,
    pub last_seen: u64,
    pub uptime: u32,
    pub load: [f32; 3],
}

impl Entry {
    pub fn format(&self) -> String {
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(e) => {
                error!("Error calculating time: {}", e);
                Duration::new(0, 0)
            }
        };

        let string = format!(
            "Entry: {} ({})\n \
             ▪ Address: {}\n \
             ▪ Platform: {}\n \
             ▪ System version: {}\n \
             ▪ Keepalive timer: {}s\n \
             ▪ Last seen: {}\n \
             ▪ Uptime: {}\n \
             ▪ Load: {:.2} {:.2} {:.2}",
            &self.system_name,
            &self.unique_id,
            &self.address,
            &self.system_platform,
            &self.system_version,
            &self.keepalive_timer,
            format_time(time.as_secs().abs_diff(self.last_seen as u64)),
            format_time(self.uptime as u64),
            self.load[0],
            self.load[1],
            self.load[2]
        );
        string
    }

    pub fn from_message(msg: message::MessageV1, address: SocketAddr) -> Self {
        Entry {
            unique_id: msg.unique_id,
            system_name: msg.system_name,
            system_platform: msg.system_platform,
            system_version: msg.system_version,
            keepalive_timer: msg.keepalive_timer,
            address: address.ip().to_string(),
            last_seen: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time is before Unix timestamp")
                .as_secs(),
            uptime: msg.uptime,
            load: msg.load,
        }
    }
}
