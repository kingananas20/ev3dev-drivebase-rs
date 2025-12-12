mod brake_mode;
mod drive;
pub mod lf;
mod ramping;
mod run;
mod speed;
mod turn;
mod utils;
mod wait;

pub use brake_mode::BrakeMode;

use crate::Motor;
use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor, sensors::ColorSensor};
use std::f64::consts::PI;

/// The `DriveBase` struct which holds all the needed fields
#[derive(Debug, Clone)]
pub struct DriveBase {
    /// The left motor of the `DriveBase`
    pub left: TachoMotor,
    /// The right motor of the `DriveBase`
    pub right: TachoMotor,
    /// Current speed of the robot
    pub current_speed: i32,
    /// left color sensor if specified, useful for line following methods
    pub left_sensor: Option<ColorSensor>,
    /// right color sensor if specified, useful for line following methods
    pub right_sensor: Option<ColorSensor>,
    /// Metadata of the left motor
    pub left_meta: Motor,
    /// Metadata of the right motor
    pub right_meta: Motor,
    /// The circumference of the wheels in mm
    pub circumference: f64,
    /// The distance between the points where both wheels touch the ground.
    pub axle_track: f64,
}

impl DriveBase {
    /// Creates a new `DriveBase` using the provided `Motor` structs for the left and right motor.
    ///
    /// # Errors
    ///
    /// Errors if the port is not used or used by another device.
    pub fn new(
        left_meta: Motor,
        right_meta: Motor,
        wheel_diameter: f64,
        axle_track: f64,
    ) -> Result<Self, Ev3Error> {
        let left = TachoMotor::get(left_meta.port)?;
        let right = TachoMotor::get(right_meta.port)?;
        let circumference = PI * wheel_diameter;
        let drivebase = Self {
            left,
            right,
            current_speed: 0,
            left_sensor: None,
            right_sensor: None,
            left_meta,
            right_meta,
            circumference,
            axle_track,
        };
        drivebase.reset()?;
        Ok(drivebase)
    }

    /// Add left and right colorsensors
    pub fn add_colorsensor(
        &mut self,
        left_sensor: ColorSensor,
        right_sensor: ColorSensor,
    ) -> &Self {
        self.left_sensor = Some(left_sensor);
        self.right_sensor = Some(right_sensor);
        self
    }
}
