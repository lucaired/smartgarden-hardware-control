#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

/// This is an inclusive interval
const LOWEST_ALLOWED_PORT: i32 = 2;
const HIGHEST_ALLOWED_PORT: i32 = 5;

#[get("/fan/<number>/on")]
fn fan_on(number: i32) -> String {
    if port_number_ok(number) {
        format!("Hello, fan {} turned on!", number)
    } else {
        format!("Hello, fan {} could not be turned on!", number)
    }
}

#[get("/off/<number>")]
fn fan_off(number: i32) -> String {
    if port_number_ok(number) {
        format!("Hello, fan {} turned off!", number)

    } else {
        format!("Hello, fan {} could not be turned off!", number)
    }
}

// TODO: make a result
fn port_number_ok(port_number: i32) -> bool {
    LOWEST_ALLOWED_PORT <= port_number && port_number <= HIGHEST_ALLOWED_PORT
}

fn main() {
    rocket::ignite().mount("/", routes![fan_on]).launch();
}