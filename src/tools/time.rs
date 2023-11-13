use chrono::prelude::*;

pub fn format_time(time_str: &str) -> String {
    let timestamp = time_str.parse::<i64>().unwrap();
    let utc = Utc.timestamp_opt(timestamp / 1000, 0).unwrap();
    let zoned: DateTime<Local> = DateTime::from(utc);
    zoned.format("%Y-%m-%d %H:%M:%S").to_string()
}
