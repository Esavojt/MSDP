pub mod client;
pub mod server;
pub mod structs;
pub mod prometheus_exporter;

pub fn format_time(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;

    if hours > 24 {
        let days = hours / 24;
        let hours = hours % 24;
        format!("{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
