//! Drive forward with a specified amount of cm

use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
use ev3_drivebase::{BrakeMode, Direction, DriveBase, Motor};

fn main() -> Result<(), Ev3Error> {
    let left_meta = Motor::new(MotorPort::OutA, Direction::Clockwise);
    let right_meta = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    let drivebase = DriveBase::new(left_meta, right_meta, 43.2, 185.)?;
    drivebase
        .set_brake_mode(BrakeMode::Hold)?
        .set_acceleration(4000)?
        .set_deceleration(4000)?;

    drivebase.drive(500, 100, true)?; // Drives 5cm forwards at 100 deg/s and stops when finished
    drivebase.drive(500, -100, true)?; // Drives 5cm backwards at 200 deg/s and stops when finished

    Ok(())
}
