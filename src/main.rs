#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

mod scheduler;
mod usb_control;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub const FAN_STATE_DATABASE: &str = "fan_state.db";

#[derive(Serialize, Deserialize, Debug)]
struct FanStatus {
   fan_number: i32,
   fan_status: i32,
}

impl FanStatus {
    fn from_tuple(tuple: (i32, i32)) -> Self {
        FanStatus {
            fan_number: tuple.0,
            fan_status: tuple.1,
        }
    }
}

#[get("/fan/<number>/on")]
fn fan_on(number: i32) -> String {
    match usb_control::fan_control(number, &"on") {
        Ok(_) => {
            let mut db = PickleDb::load(FAN_STATE_DATABASE, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();
            db.set(&number.to_string(), &1).unwrap();
            format!("Hello, fan {} turned on!", number)
        },
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned on!", number)
        }
    }
}

#[get("/fan/<number>/off")]
fn fan_off(number: i32) -> String {
    match usb_control::fan_control(number, &"off") {
        Ok(_) => { 
            let mut db = PickleDb::load(FAN_STATE_DATABASE, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();
            db.set(&number.to_string(), &0).unwrap();
            format!("Hello, fan {} turned off!", number)
        },
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned off!", number)
        }
    }
}

#[get("/fan")]
fn all_fan_status() -> String {
    let db = PickleDb::load(FAN_STATE_DATABASE, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();
    let all_fan = vec![2,3,4,5];
    let all_fan_state: Vec<FanStatus> = all_fan
        .into_iter()
        .map(|fan_number| {
            let state = db.get::<i32>(&fan_number.to_string()).unwrap();
            (fan_number, state)
    })
    .map(FanStatus::from_tuple)
    .collect::<Vec<FanStatus>>();

    format!("{:?}", all_fan_state)
}

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