use std::io::{self, Write};

pub fn ok(message: &str) {
    println!("\u{2713} {message}");
}

pub fn warn(message: &str) {
    println!("\u{26A0} {message}");
}

pub fn err(message: &str) {
    let _ = writeln!(io::stderr(), "\u{2717} {message}");
}

pub fn info(message: &str) {
    println!("\u{2192} {message}");
}

#[allow(dead_code)]
pub fn step(message: &str, current: Option<u32>, total: Option<u32>) {
    match (current, total) {
        (Some(c), Some(t)) => println!("[{c}/{t}] {message}"),
        _ => println!("[\u{00B7}] {message}"),
    }
}
