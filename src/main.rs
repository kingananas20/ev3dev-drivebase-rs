//! Simple program purely for testing

use std::{thread::sleep, time::Duration};

use ev3_drivebase::{
    BrakeMode, Direction, DriveBase, Motor,
    ev3dev_lang_rust::{Ev3Error, motors::MotorPort},
};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let mut drivebase = DriveBase::new(left, right, 43.)?;
    drivebase.set_brake_mode(BrakeMode::Hold)?;

    drivebase.drive(500 * 2, 200, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500 * 2, -200, true)?;
    sleep(Duration::from_secs(2));
    println!("done driving");

    Ok(())
}
