use crate::{usb_control, FAN_STATE_DATABASE};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct FanStatus {
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
pub fn fan_on(number: i32) -> String {
    match usb_control::fan_control(number, &"on") {
        Ok(_) => {
            let mut db = PickleDb::load(
                FAN_STATE_DATABASE,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )
            .unwrap();
            db.set(&number.to_string(), &1).unwrap();
            format!("Hello, fan {} turned on!", number)
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned on!", number)
        }
    }
}

#[get("/fan/<number>/off")]
pub fn fan_off(number: i32) -> String {
    match usb_control::fan_control(number, &"off") {
        Ok(_) => {
            let mut db = PickleDb::load(
                FAN_STATE_DATABASE,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )
            .unwrap();
            db.set(&number.to_string(), &0).unwrap();
            format!("Hello, fan {} turned off!", number)
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            format!("Hello, fan {} could not be turned off!", number)
        }
    }
}

#[get("/fan")]
pub fn all_fan_status() -> Json<Vec<FanStatus>> {
    let db = PickleDb::load(
        FAN_STATE_DATABASE,
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    )
    .unwrap();
    let all_fan = vec![2, 3, 4, 5];
    let all_fan_state: Vec<FanStatus> = all_fan
        .into_iter()
        .map(|fan_number| {
            let state = db.get::<i32>(&fan_number.to_string()).unwrap();
            (fan_number, state)
        })
        .map(FanStatus::from_tuple)
        .collect::<Vec<FanStatus>>();
    
        // TODO: return JSON
        Json(all_fan_state)
}


