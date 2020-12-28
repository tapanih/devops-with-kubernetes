#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref REDIRECT_URL: String = std::env::var("REDIRECT_URL")
        .expect("REDIRECT_URL not found");
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

pub struct StateTodos {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl StateTodos {
    pub fn new() -> StateTodos {
        let todos = Arc::new(Mutex::new(vec![
            Todo::new(String::from("Todo 1")),
            Todo::new(String::from("Another todo")),
        ]));

        StateTodos { todos }
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

#[post("/todos", format= "application/x-www-form-urlencoded", data = "<user_input>")]
fn post_todos_form(user_input: Form<UserInput>, state: State<StateTodos>) -> Redirect {
    Arc::clone(&state.todos).lock().unwrap().push(Todo { content: user_input.content.clone() });
    Redirect::to(&REDIRECT_URL[..])
}

#[post("/todos", format= "application/json", data= "<todo>")]
fn post_todos(todo: Json<Todo>, state: State<StateTodos>) -> &'static str {
    Arc::clone(&state.todos).lock().unwrap().push(todo.into_inner());
    "Todo added"
}

#[get("/todos")]
pub fn get_todos<'a>(state: State<'a, StateTodos>) -> Json<Todos> {
    Json(Todos { todos: Arc::clone(&state.inner().todos).lock().unwrap().to_vec() })
}

fn main() {
    rocket::ignite()
        .manage(StateTodos::new())
        .mount("/", routes![get_todos, post_todos, post_todos_form])
        .launch();
}
