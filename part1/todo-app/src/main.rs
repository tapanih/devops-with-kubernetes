#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::State;
use std::env;

const DAY_IN_SECONDS: u64 = 86400;

lazy_static! {
    static ref IMAGE_FILE: String = env::var("IMAGE_FILE")
        .expect("IMAGE_FILE not found");
}

struct DaysSinceEpoch(AtomicU64);

fn get_new_image() -> Result<bytes::Bytes, String> {
    let response = reqwest::blocking::get("https://picsum.photos/1200");
    return match response {
        Err(_)   => return Err("Could not get image".to_string()),
        Ok(resp) => match resp.bytes() {
            Err(_)    => return Err("Could not convert to bytes".to_string()),
            Ok(bytes) => Ok(bytes)
        }
    };
}

#[get("/")]
fn index(state: State<DaysSinceEpoch>) -> Result<NamedFile, NotFound<String>> {
    let old_days_since_epoch = state.0.load(Ordering::Relaxed);
    let new_days_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / DAY_IN_SECONDS;
    if new_days_since_epoch - old_days_since_epoch > 0 { // if day has changed
        println!("Fetching a new image...");
        match get_new_image() {
            Err(e) => println!("{}", e),
            Ok(b)  => match std::fs::write(&IMAGE_FILE[..], b) {
                Err(_) => println!("Could not write to file."),
                Ok(_)  => state.0.store(new_days_since_epoch, Ordering::Relaxed),
            }
        }
    }
    NamedFile::open(&IMAGE_FILE[..]).map_err(|_| NotFound("Image not found".to_string()))
}

fn main() {
    let bytes = get_new_image().expect("Error: ");
    std::fs::write(&IMAGE_FILE[..], bytes).expect("Could not write to file.");
    let days_since_epoch = AtomicU64::new(
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / DAY_IN_SECONDS
    );
    rocket::ignite()
        .manage(DaysSinceEpoch(days_since_epoch))
        .mount("/", routes![index]).launch();
}
