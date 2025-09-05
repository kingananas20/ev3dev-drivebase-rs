//! Simple program purely for testing

use ev3::{Direction, DriveBase, Motor, ev3dev_lang_rust::motors::MotorPort};
use ev3dev_lang_rust::Ev3Error;

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let drivebase = DriveBase::new(left, right, 43.)?;

    drivebase.drive(300, 50, true)?;
    println!("done driving");

    Ok(())
}
