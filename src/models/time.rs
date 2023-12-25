use chrono::{prelude::*, Duration};

pub fn current_timestamp() -> i64 {
    return Utc::now().timestamp();
}

pub fn time_delta(from: i64, to: Option<i64>) -> i64 {
    return to.expect("timedelta") - from;
}

pub fn to_human_date(timestamp: i64) -> String {
    let dt = DateTime::from_timestamp(timestamp, 0).expect("invalid timestamp");
    return dt.format("%d-%m-%Y").to_string();
}

pub fn duration(seconds: i64) -> String {
    let d = Duration::seconds(seconds);
    return format!(
        "{}-d {}-h {}-m",
        d.num_days(),
        d.num_hours() % 24,
        d.num_minutes() % 60
    );
}
