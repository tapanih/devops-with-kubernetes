#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::State;
use std::env;
use rocket_contrib::templates::Template;

const DAY_IN_SECONDS: u64 = 86400;

lazy_static! {
    static ref IMAGE_FILE: String = env::var("IMAGE_FILE")
        .expect("IMAGE_FILE not found");
    static ref TODO_SERVICE_URL: String = env::var("TODO_SERVICE_URL")
        .expect("TODO_SERVICE_URL not found");
    static ref TODO_POST_URL: String = env::var("TODO_POST_URL")
        .expect("TODO_POST_URL not found");
}

struct DaysSinceEpoch(AtomicU64);

fn get_new_image() -> Result<bytes::Bytes, String> {
    let response = reqwest::blocking::get("http://picsum.photos/1200");
    return match response {
        Err(_)   => return Err("Could not get image".to_string()),
        Ok(resp) => match resp.bytes() {
            Err(_)    => return Err("Could not convert to bytes".to_string()),
            Ok(bytes) => Ok(bytes)
        }
    };
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub content: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Context {
    todo_post_url: &'static str,
    todos: Vec<Todo>
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Todos {
    todos: Vec<Todo>
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo { content }
    }
}

#[get("/")]
fn index() -> Template {
    let todos: Todos = match reqwest::blocking::get(&TODO_SERVICE_URL[..]) {
        Err(_) => {
            println!("Error: could not fetch todos from the service");
            Todos{ todos: Vec::new() }
        },
        Ok(response) => response.json().unwrap_or_else(|_| {
            println!("Error: could not parse json");
            Todos{ todos: Vec::new() }
        })
    };

    Template::render("index", Context { todos: todos.todos, todo_post_url: &TODO_POST_URL[..] })
}

#[get("/daily")]
fn daily(state: State<DaysSinceEpoch>) -> Result<NamedFile, NotFound<String>> {
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
    let bytes = get_new_image().expect("Error");
    std::fs::write(&IMAGE_FILE[..], bytes).expect("Could not write to file.");
    let days_since_epoch = AtomicU64::new(
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / DAY_IN_SECONDS
    );
    rocket::ignite()
        .manage(DaysSinceEpoch(days_since_epoch))
        .attach(Template::fairing())
        .mount("/", routes![index, daily]).launch();
}
