use chrono::{DateTime, FixedOffset, Utc};

pub fn utc8_offset() -> FixedOffset {
    FixedOffset::east_opt(8 * 60 * 60).expect("UTC+8 offset must be valid")
}

pub fn utc8_now() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&utc8_offset())
}

pub fn now_iso() -> String {
    utc8_now().format("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

pub fn now_task_id() -> String {
    utc8_now().format("%Y-%m-%dT%H-%M-%S").to_string()
}

pub fn normalize_text(value: &str) -> String {
    value.trim().to_string()
}
