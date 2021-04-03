use crate::{follower::set_state_to_fan, FAN_STATE_DATABASE};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rocket_contrib::json::Json;
use serde::Serialize;

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

fn set_state_to_db(number: i32, state: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = PickleDb::load(
        FAN_STATE_DATABASE,
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    )?;
    // TODO: set according to function
    db.set(&number.to_string(), &state)?;
    Ok(())
}

#[get("/fan/<number>/on")]
pub fn fan_on(number: i32) -> Result<String, Box<dyn std::error::Error>> {
    set_state_to_fan(&"on")?;
    set_state_to_db(number, 1)?;
    Ok(format!("Hello, fan {} turned on!", number))
}

#[get("/fan/<number>/off")]
pub fn fan_off(number: i32) -> Result<String, Box<dyn std::error::Error>> {
    set_state_to_fan(&"off")?;
    set_state_to_db(number, 0)?;
    Ok(format!("Hello, fan {} turned off!", number))
}

#[get("/fan")]
pub fn all_fan_status() -> Result<Json<Vec<FanStatus>>, Box<dyn std::error::Error>> {
    let db = PickleDb::load(
        FAN_STATE_DATABASE,
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    )?;
    let all_fan_state: Vec<FanStatus> = crate::ALL_FAN
        .iter()
        .map(|fan_number| {
            // we default to a faulty state if we could not get the state from the database
            let state = db.get::<i32>(&fan_number.to_string()).unwrap_or(-1);
            (*fan_number, state)
        })
        .map(FanStatus::from_tuple)
        .collect::<Vec<FanStatus>>();

    Ok(Json(all_fan_state))
}
