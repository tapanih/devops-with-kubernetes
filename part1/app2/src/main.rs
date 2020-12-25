#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

fn main() {
    rocket::ignite().launch();
}
