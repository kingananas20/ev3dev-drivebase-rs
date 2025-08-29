mod speed;
mod utils;

use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor};

use crate::Motor;

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
}

impl DriveBase {
    /// Creates a new `DriveBase` using the provided `Motor` structs for the left and right motor.
    ///
    /// # Errors
    ///
    /// Errors if the port is not used or used by another device.
    pub fn new(left_meta: Motor, right_meta: Motor) -> Result<Self, Ev3Error> {
        let left = TachoMotor::get(left_meta.port)?;
        let right = TachoMotor::get(right_meta.port)?;
        let drivebase = Self {
            left,
            right,
            left_meta,
            right_meta,
        };
        drivebase.reset()?;
        Ok(drivebase)
    }

    /// Runs forever with the given speed.
    ///
    /// # Errors
    ///
    /// Errors if it can't write to the corresponding sysfs file
    pub fn run_forever(&self, speed: i32) -> Result<&Self, Ev3Error> {
        self.set_speed(speed)?;

        self.left.run_forever()?;
        self.right.run_forever()?;
        Ok(self)
    }
}
