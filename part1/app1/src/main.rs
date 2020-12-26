#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use chrono::prelude::{Utc};
use uuid::Uuid;

lazy_static! {
    static ref UUID: Uuid = Uuid::new_v4();
}

#[get("/")]
fn index() -> String {
    format!("{}: {}", Utc::now(), *UUID)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
