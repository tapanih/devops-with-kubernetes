use chrono::prelude::{Utc};
use std::{thread, time};
use std::env;
use std::fs;

fn main() {
    let filename = env::var("TIMESTAMP_FILE")
        .expect("TIMESTAMP_FILE not found");
    loop {
        fs::write(&filename, format!("{}", Utc::now()))
            .unwrap_or_else(|_| {
                println!("Could not write to file. Retrying...");
            });
        thread::sleep(time::Duration::from_secs(5));
    }
}
