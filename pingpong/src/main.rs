#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use postgres::{Client, NoTls};

lazy_static! {
    static ref POSTGRES_URL: String = std::env::var("POSTGRES_URL")
        .expect("POSTGRES_URL not found");
}

fn initialize_database() {
    let mut client: Client = Client::connect(&POSTGRES_URL[..], NoTls)
        .expect("could not connect to database");
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS pong ( ping INTEGER );
        INSERT INTO pong (ping) VALUES (0);
        CREATE OR REPLACE RULE no_insert AS ON INSERT TO pong DO INSTEAD NOTHING;
    ").unwrap();
}

fn get_pongs_from_db(client: &mut Client) -> i32 {
    return match client.query_one("SELECT ping FROM pong", &[]) {
        Ok(row) => row.get("ping"),
        Err(_) => 0,
    };
}

#[get("/pongs")]
fn pong() -> String {
    let mut client: Client = Client::connect(&POSTGRES_URL[..], NoTls)
        .expect("could not connect to database");
    format!("{}", get_pongs_from_db(&mut client))
}

#[get("/pingpong")]
fn index() -> String {
    let mut client: Client = Client::connect(&POSTGRES_URL[..], NoTls)
        .expect("could not connect to database");
    client.batch_execute("UPDATE pong SET ping = ping + 1;").ok();
    format!("pong {}", get_pongs_from_db(&mut client))
}

fn main() {
    initialize_database();
    rocket::ignite()
      .mount("/", routes![index, pong])
      .launch();
}
