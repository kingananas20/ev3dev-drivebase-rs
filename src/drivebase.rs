//! Core drivebase functionality for controlling a two-wheeled robot.

mod brake_mode;
mod drive;
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

/// A two-wheeled robot drivebase controller.
///
/// The `DriveBase` provides high-level control over a differential drive robot,
/// handling the complex calculations needed for precise movement and turning.
///
/// # Physical Parameters
///
/// The drivebase needs to know the physical dimensions of your robot:
///
/// - **Wheel diameter**: The diameter of your wheels in millimeters
/// - **Axle track**: The distance between the contact points of the left and right wheels in millimeters
///
/// These measurements are critical for accurate distance and angle calculations.
///
/// # Examples
///
/// Basic setup and movement:
///
/// ```no_run
/// use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
/// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
///
/// fn main() -> Result<(), Ev3Error> {
///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
///
///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
///     drivebase.set_brake_mode(BrakeMode::Hold)?;
///
///     // Drive 200mm forward at 300 deg/s
///     drivebase.drive(300, 200, true)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DriveBase {
    /// The left motor of the drivebase.
    pub left: TachoMotor,
    /// The right motor of the drivebase.
    pub right: TachoMotor,
    /// Current speed of the robot in degrees per second.
    pub current_speed: i32,
    /// Left color sensor if specified, useful for line following methods.
    pub left_sensor: Option<ColorSensor>,
    /// Right color sensor if specified, useful for line following methods.
    pub right_sensor: Option<ColorSensor>,
    /// Metadata of the left motor.
    pub left_meta: Motor,
    /// Metadata of the right motor.
    pub right_meta: Motor,
    /// The circumference of the wheels in millimeters.
    pub circumference: f64,
    /// The distance between the points where both wheels touch the ground in millimeters.
    pub axle_track: f64,
}

impl DriveBase {
    /// Creates a new `DriveBase` using the provided motor configurations.
    ///
    /// # Parameters
    ///
    /// - `left_meta`: Configuration for the left motor
    /// - `right_meta`: Configuration for the right motor
    /// - `wheel_diameter`: Diameter of the wheels in millimeters
    /// - `axle_track`: Distance between wheel contact points in millimeters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The specified motor ports are not connected
    /// - The ports are being used by another device
    /// - The motors cannot be initialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///
    ///     // Create drivebase with 43.2mm diameter wheels and 185mm axle track
    ///     let drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Adds left and right color sensors to the drivebase.
    ///
    /// Color sensors can be used for line following and other applications
    /// that require detecting surface colors or light intensity.
    ///
    /// # Parameters
    ///
    /// - `left_sensor`: The color sensor on the left side of the robot
    /// - `right_sensor`: The color sensor on the right side of the robot
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    /// use ev3_drivebase::ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left_motor = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right_motor = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left_motor, right_motor, 43.2, 185.0)?;
    ///
    ///     let left_sensor = ColorSensor::get(SensorPort::In1)?;
    ///     let right_sensor = ColorSensor::get(SensorPort::In2)?;
    ///
    ///     drivebase.add_colorsensor(left_sensor, right_sensor);
    ///
    ///     Ok(())
    /// }
    /// ```
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
