#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

mod scheduler;
mod usb_control;
mod handler;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use handler::{fan_on, fan_off, all_fan_status};

pub const FAN_STATE_DATABASE: &str = "fan_state.db";

fn main() {
    let mut db = PickleDb::new(FAN_STATE_DATABASE, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
    // turn all fans off and set their state to off at startup
    let all_fan = vec![2,3,4,5];
    all_fan.into_iter().for_each(|fan_number| {
        db.set(&fan_number.to_string(), &0).unwrap();
        usb_control::fan_control(fan_number, &"off").unwrap();
    });
    rocket::ignite().mount("/", routes![fan_on, fan_off, all_fan_status]).launch();
}