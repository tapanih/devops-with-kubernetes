#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;
use std::env;
use std::fs;

struct Counter(AtomicUsize);

#[get("/pongs")]
fn pong(counter: State<Counter>) -> String {
    format!("{}", counter.0.load(Ordering::Relaxed))
}

#[get("/pingpong")]
fn index(counter: State<Counter>) -> String {
    let count = counter.0.fetch_add(1, Ordering::Relaxed);
    let filename = env::var("PINGPONG_FILE")
        .expect("PINGPONG_FILE not found");
    fs::write(&filename, count.to_string())
        .unwrap_or_else(|_| {
            println!("Could not write to file.");
        });
    format!("pong {}", count.to_string())
}

fn main() {
    rocket::ignite()
      .mount("/", routes![index, pong])
      .manage(Counter(AtomicUsize::new(0))).launch();
}
