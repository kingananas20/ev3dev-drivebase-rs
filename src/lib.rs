//! # ev3-drivebase
//!
//! A high-level `DriveBase` abstraction for LEGO Mindstorms EV3 robots running ev3dev.
//!
//! This crate provides an easy-to-use interface for controlling a two-wheeled robot,
//! similar to the `DriveBase` classes found in the Python and `MicroPython` ev3dev libraries.
//!
//! ## Features
//!
//! - **Simple movement control**: Drive forward/backward and turn with precise distance and angle control
//! - **Configurable brake modes**: Choose between Coast, Brake, and Hold modes
//! - **Acceleration control**: Set custom acceleration and deceleration rates
//! - **Color sensor support**: Optional integration with color sensors for line following
//! - **Type-safe API**: Leverage Rust's type system for safer robot control
//!
//! ## Quick Start
//!
//! ```no_run
//! use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
//! use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
//!
//! fn main() -> Result<(), Ev3Error> {
//!     // Define motor configuration
//!     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
//!     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
//!
//!     // Create drivebase with wheel diameter (43.2mm) and axle track (185mm)
//!     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
//!
//!     // Configure behavior
//!     drivebase
//!         .set_brake_mode(BrakeMode::Hold)?
//!         .set_acceleration(4000)?
//!         .set_deceleration(4000)?;
//!
//!     // Drive 500mm forward at 100 deg/s
//!     drivebase.drive(100, 500, true)?;
//!
//!     // Turn 90 degrees in place
//!     drivebase.turn(200, 90, None)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Motor Direction
//!
//! When creating a `DriveBase`, you need to specify the direction each motor should turn
//! to make the robot move forward. This depends on how your motors are mounted:
//!
//! - If the motor shaft points toward the front of the robot, use `Direction::Clockwise`
//! - If the motor shaft points toward the back of the robot, use `Direction::CounterClockwise`
//!
//! ## Units
//!
//! - **Distances**: millimeters (mm)
//! - **Speeds**: degrees per second (deg/s)
//! - **Angles**: degrees
//! - **Acceleration/Deceleration**: degrees per second squared (deg/s²)
//!
//! ## Color Sensors
//!
//! You can optionally add color sensors to the drivebase for line following applications:
//!
//! ```no_run
//! # use ev3_drivebase::{DriveBase, Motor, Direction};
//! # use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
//! use ev3_drivebase::ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
//!
//! # fn main() -> Result<(), Ev3Error> {
//! # let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
//! # let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
//! # let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
//! let left_sensor = ColorSensor::get(SensorPort::In1)?;
//! let right_sensor = ColorSensor::get(SensorPort::In2)?;
//!
//! drivebase.add_colorsensor(left_sensor, right_sensor);
//! # Ok(())
//! # }
//! ```

mod drivebase;
mod motor;

pub use drivebase::{BrakeMode, DriveBase};
pub use ev3dev_lang_rust;
pub use motor::{Direction, Motor};
