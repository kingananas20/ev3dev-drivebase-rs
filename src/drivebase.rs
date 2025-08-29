mod drive;
mod run;
mod speed;
mod utils;
mod wait;

use crate::Motor;
use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor};
use std::f64::consts::PI;

/// The `DriveBase` struct which holds all the needed fields
#[derive(Debug, Clone)]
pub struct DriveBase {
    /// The left motor of the `DriveBase`
    pub left: TachoMotor,
    /// The right motor of the `DriveBase`
    pub right: TachoMotor,
    /// Metadata of the left motor
    pub left_meta: Motor,
    /// Metadata of the right motor
    pub right_meta: Motor,
    /// The circumference of the wheels in mm
    pub circumference: f64,
}

impl DriveBase {
    /// Creates a new `DriveBase` using the provided `Motor` structs for the left and right motor.
    ///
    /// # Errors
    ///
    /// Errors if the port is not used or used by another device.
    pub fn new(left_meta: Motor, right_meta: Motor, wheel_diameter: f64) -> Result<Self, Ev3Error> {
        let left = TachoMotor::get(left_meta.port)?;
        let right = TachoMotor::get(right_meta.port)?;
        let circumference = PI * wheel_diameter;
        let drivebase = Self {
            left,
            right,
            left_meta,
            right_meta,
            circumference,
        };
        drivebase.reset()?;
        Ok(drivebase)
    }
}
