#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure_derive;
extern crate serde_derive;

mod handler;
mod scheduler;
mod usb_control;

use handler::{
    static_rocket_route_info_for_all_fan_status, static_rocket_route_info_for_fan_off,
    static_rocket_route_info_for_fan_on,
};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub const FAN_STATE_DATABASE: &str = "fan_state.db";
pub const ALL_FAN: [i32; 4] = [2, 3, 4, 5];

fn main() {
    let mut db = PickleDb::new(
        FAN_STATE_DATABASE,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );
    // turn all fans off and set their state to off at startup

    ALL_FAN.iter().for_each(|fan_number| {
        db.set(&fan_number.to_string(), &0).unwrap();
        usb_control::fan_control(*fan_number, &"off").unwrap();
    });
    rocket::ignite()
        .mount("/", routes![fan_on, fan_off, all_fan_status])
        .launch();
}
