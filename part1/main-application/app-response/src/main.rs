#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use uuid::Uuid;
use std::env;
use std::fs;

lazy_static! {
    static ref UUID: Uuid = Uuid::new_v4();
    static ref TIMESTAMP_FILE: String = env::var("TIMESTAMP_FILE")
        .expect("TIMESTAMP_FILE not found");
    static ref PINGPONG_URL: String = env::var("PINGPONG_URL")
        .expect("PINGPONG_URL not found");
}

#[get("/main")]
fn index() -> String {
    let pongs = match reqwest::blocking::get(&PINGPONG_URL[..]) {
        Ok(response) => response.text().unwrap_or("0".to_string()),
        Err(_)       => "0".to_string()
    };
    let timestamp = fs::read_to_string(&TIMESTAMP_FILE[..]).unwrap_or("".to_string());
    format!("{}: {}\nPing / Pongs: {}", timestamp.trim(), *UUID, pongs.trim())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
