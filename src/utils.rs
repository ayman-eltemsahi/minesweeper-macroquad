use chrono::offset::Utc;

pub fn current_time_seconds() -> i64 {
    Utc::now().timestamp()
}

pub fn get_time_diff(start: i64) -> (i32, i32) {
    let diff = Utc::now().timestamp() - start;
    let minutes = (diff / 60) as i32;
    let seconds = (diff % 60) as i32;
    (minutes, seconds)
}
