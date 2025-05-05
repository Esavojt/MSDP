pub mod server;
pub mod structs;

pub fn format_time(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;

    if hours > 24 {
        let days = hours / 60;
        let hours = hours % 60;
        format!("{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
