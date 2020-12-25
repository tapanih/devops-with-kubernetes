use std::{thread, time};
use chrono::prelude::{Utc};
use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    loop { 
        println!("{}: {}", Utc::now(), my_uuid);
        thread::sleep(time::Duration::from_secs(5));
    }
}
