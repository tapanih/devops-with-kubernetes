#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;

struct Counter(AtomicUsize);

#[get("/")]
fn index(counter: State<Counter>) -> String {
    let count = counter.0.fetch_add(1, Ordering::Relaxed);
    format!("pong {}", count.to_string())
}

fn main() {
    rocket::ignite()
      .mount("/", routes![index])
      .manage(Counter(AtomicUsize::new(0))).launch();
}
