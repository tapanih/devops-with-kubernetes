#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;
use postgres::{Client, NoTls};

lazy_static! {
    static ref REDIRECT_URL: String = std::env::var("REDIRECT_URL")
        .expect("REDIRECT_URL not found");
    static ref POSTGRES_URL: String = std::env::var("POSTGRES_URL")
        .expect("POSTGRES_URL not found");
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub content: String,
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo { content }
    }
}

#[derive(serde::Serialize)]
pub struct Todos {
    pub todos: Vec<Todo>,
}

#[derive(FromForm)]
struct UserInput {
    content: String
}

fn get_db_connection() -> Client {
    return Client::connect(&POSTGRES_URL[..], NoTls)
        .expect("could not connect to database");
}

fn get_todos_from_db(client: &mut Client) -> Vec<Todo> {
    return client.query("SELECT content FROM todos", &[]).unwrap()
        .into_iter().map(|row| Todo { content: row.get(0) }).collect::<Vec<Todo>>();
}

fn initialize_database() {
    let mut client = get_db_connection();
    client.batch_execute("CREATE TABLE IF NOT EXISTS todos ( content VARCHAR(140) );").unwrap();
}

#[post("/todos", format= "application/x-www-form-urlencoded", data = "<user_input>")]
fn post_todos_form(user_input: Form<UserInput>) -> Redirect {
    let mut client = get_db_connection();
    match client.execute("INSERT INTO todos VALUES ($1)", &[&user_input.content]) {
        Ok(_) => println!("Todo added"),
        Err(_) => println!("Invalid todo"),
    }
    Redirect::to(&REDIRECT_URL[..])
}

#[post("/todos", format= "application/json", data= "<todo>")]
fn post_todos(todo: Json<Todo>) -> Result<&'static str, BadRequest<&'static str>> {
    let mut client = get_db_connection();
    match client.execute("INSERT INTO todos VALUES ($1)", &[&todo.into_inner().content]) {
        Ok(_)  => {
            println!("Todo added");
            Ok("Todo added")
        }
        Err(_) => {
            println!("Invalid todo");
            Err(BadRequest(Some("Invalid todo")))
        }
    }
}

#[get("/todos")]
pub fn get_todos() -> Json<Todos> {
    Json(Todos { todos: get_todos_from_db(&mut get_db_connection()) })
}

fn main() {
    initialize_database();
    rocket::ignite()
        .mount("/", routes![get_todos, post_todos, post_todos_form])
        .launch();
}
