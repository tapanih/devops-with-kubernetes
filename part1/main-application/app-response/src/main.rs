#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use uuid::Uuid;
use std::env;
use std::fs;

lazy_static! {
    static ref UUID: Uuid = Uuid::new_v4();
    static ref PINGPONG_FILE: String = env::var("PINGPONG_FILE")
        .expect("PINGPONG_FILE not found");
    static ref TIMESTAMP_FILE: String = env::var("TIMESTAMP_FILE")
        .expect("TIMESTAMP_FILE not found");
}

#[get("/")]
fn index() -> String {
    let counter = fs::read_to_string(&PINGPONG_FILE[..]).unwrap_or("0".to_string());
    let timestamp = fs::read_to_string(&TIMESTAMP_FILE[..]).unwrap_or("".to_string());
    format!("{}: {}\nPing / Pongs: {}", timestamp.trim(), *UUID, counter.trim())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
