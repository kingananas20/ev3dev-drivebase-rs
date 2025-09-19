//! Simple program purely for testing

use ev3_drivebase::{
    BrakeMode, Direction, DriveBase, Motor,
    ev3dev_lang_rust::{Ev3Error, motors::MotorPort},
};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let drivebase = DriveBase::new(left, right, 43., 185.)?;
    drivebase.set_brake_mode(BrakeMode::Hold)?;

    drivebase.turn(500, 90, None)?;
    sleep(Duration::from_secs(2));
    drivebase.turn(500, -90, None)?;

    /*drivebase.drive(500, 50, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -50, true)?;
    sleep(Duration::from_secs(2));*/

    println!("done driving");

    drivebase.set_brake_mode(BrakeMode::Coast)?;

    Ok(())
}
