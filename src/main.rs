#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use]
extern crate failure_derive;

use std::process::{Command, Output};

/// This is an inclusive interval
const LOWEST_ALLOWED_PORT: i32 = 2;
const HIGHEST_ALLOWED_PORT: i32 = 5;

#[derive(Debug, Fail)]
pub enum UsbControlError {
    #[fail(display = "Invalid port number : {}", fan_number)]
    InvalidFanNumber { fan_number: i32 },
    #[fail(display = "Could not execute : {}", command)]
    CommandError { command: String },
}

/// Parse the arguments provided
/// `(on || off)` as switch argument
/// `[2,5]` as fan_numer argument
pub fn fan_control(fan_number: i32, switch: &str) -> Result<Output, UsbControlError> {
    fan_number_ok(fan_number)?;
    let command = format!(
        "sudo uhubctl -a {} -p {}",
        switch, fan_number
    );
    execute(command)
}

fn execute(command: String) -> Result<Output, UsbControlError> {
    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(output) => Ok(output),
        Err(_err) => Err(UsbControlError::CommandError { command }),
    }
}

fn fan_number_ok(fan_number: i32) -> Result<(), UsbControlError> {
    if LOWEST_ALLOWED_PORT <= fan_number && fan_number <= HIGHEST_ALLOWED_PORT {
        Ok(())
    } else {
        Err(UsbControlError::InvalidFanNumber { fan_number })
    }
}


#[get("/fan/<number>/on")]
fn fan_on(number: i32) -> String {
    match fan_number_ok(number) {
        Ok(()) => {
            match fan_control(number, &"on") {
                Ok(_) => format!("Hello, fan {} turned on!", number),
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    format!("Hello, fan {} could not be turned on!", number)
                }
            }
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned on!", number)
        }
    }
}

#[get("/fan/<number>/off")]
fn fan_off(number: i32) -> String {
    match fan_number_ok(number) {
        Ok(()) => format!("Hello, fan {} turned off!", number),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned off!", number)
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![fan_on, fan_off]).launch();
}