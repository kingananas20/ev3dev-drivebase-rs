//! Simple program purely for testing

use ev3::{Direction, DriveBase, Motor, ev3dev_lang_rust::motors::MotorPort};
use ev3dev_lang_rust::Ev3Error;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let drivebase = DriveBase::new(left, right)?;
    drivebase.reset()?;

    drivebase.run_forever(500)?;

    sleep(Duration::from_secs(1));

    Ok(())
}
