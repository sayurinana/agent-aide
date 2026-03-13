use chrono::Local;

pub fn now_iso() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

pub fn now_task_id() -> String {
    Local::now().format("%Y-%m-%dT%H-%M-%S").to_string()
}

pub fn normalize_text(value: &str) -> String {
    value.trim().to_string()
}
