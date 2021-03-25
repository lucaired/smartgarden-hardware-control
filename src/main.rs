#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/fan/<number>/on")]
fn hello(number: u8) -> String {
    format!("Hello, fan {} turned on!", number)
}

#[get("/fan/<number>/off")]
fn hello(number: u8) -> String {
    format!("Hello, fan {} turned off!", number)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}