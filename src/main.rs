//! Simple program purely for testing

use std::{thread::sleep, time::Duration};

use ev3_drivebase::{
    Direction, DriveBase, Motor,
    ev3dev_lang_rust::{Ev3Error, motors::MotorPort},
};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let drivebase = DriveBase::new(left, right, 43.)?;

    drivebase.drive(500, 400, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -400, false)?;
    sleep(Duration::from_millis(500));
    println!("done driving");

    Ok(())
}
