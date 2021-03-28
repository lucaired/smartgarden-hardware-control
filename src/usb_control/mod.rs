use std::process::{Command, Output};

#[derive(Debug, Fail)]
pub enum UsbControlError {
    #[fail(display = "Invalid port number : {}", fan_number)]
    InvalidFanNumber { fan_number: i32 },
    #[fail(display = "Could not execute : {}", command)]
    CommandError { command: String },
}

/// This is an inclusive interval and
/// sets the bounds for the usb port numbers.
/// It defaults to [2,5].
pub fn fan_number_ok(fan_number: i32) -> Result<(), UsbControlError> {
    let lowest_allowed_port: i32 = *crate::ALL_FAN.first().unwrap_or(&2);
    let highest_allowed_port: i32 = *crate::ALL_FAN.last().unwrap_or(&5);
    if lowest_allowed_port <= fan_number && fan_number <= highest_allowed_port {
        Ok(())
    } else {
        Err(UsbControlError::InvalidFanNumber { fan_number })
    }
}

/// Parse the arguments provided
/// `(on || off)` as switch argument
/// `[2,5]` as fan_numer argument
pub fn fan_control(fan_number: i32, switch: &str) -> Result<Output, UsbControlError> {
    fan_number_ok(fan_number)?;
    let command = format!("sudo uhubctl -a {} -p {}", switch, fan_number);
    execute(command)
}

fn execute(command: String) -> Result<Output, UsbControlError> {
    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(output) => Ok(output),
        Err(_err) => Err(UsbControlError::CommandError { command }),
    }
}
