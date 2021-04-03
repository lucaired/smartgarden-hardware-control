#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate failure_derive;
extern crate serde_derive;

mod follower;
mod http;
mod scheduler;

use follower::set_state_to_fan;
use http::{
    static_rocket_route_info_for_all_fan_status, static_rocket_route_info_for_fan_off,
    static_rocket_route_info_for_fan_on,
};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

// TODO: move to .env
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
        // we need the database
        match set_state_to_fan(&"off") {
            Ok(_) => db
                .set(&fan_number.to_string(), &0)
                .expect("Could not write to database"),
            Err(err) => {
                db.set(&fan_number.to_string(), &-1)
                    .expect("Could not write to database");
                format!("Could not set state to fan {} {:?}", fan_number, err);
            }
        };
    });
    rocket::ignite()
        .mount("/", routes![fan_on, fan_off, all_fan_status])
        .launch();
}
