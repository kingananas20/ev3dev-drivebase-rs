//! Simple program purely for testing

use ev3_drivebase::{
    BrakeMode, Direction, DriveBase, Motor,
    ev3dev_lang_rust::{Ev3Error, motors::MotorPort},
};
use ev3dev_lang_rust::sensors::SensorPort;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::CounterClockwise);
    let right = Motor::new(MotorPort::OutC, Direction::Clockwise);
    let drivebase = DriveBase::new(left, right, 43.2, 185.)?;
    drivebase.set_brake_mode(BrakeMode::Hold)?;

    /*drivebase.turn(500, 90, 142.5)?;
    sleep(Duration::from_secs(1));
    drivebase.turn(-500, 90, 142.5)?;
    sleep(Duration::from_secs(1));
    drivebase.turn(500, -90, 142.5)?;
    sleep(Duration::from_secs(1));
    drivebase.turn(-500, -90, 142.5)?;*/

    /*drivebase.turn(500, 90, None)?;
    sleep(Duration::from_secs(2));
    drivebase.turn(500, -90, None)?;*/

    /*drivebase.drive(500, 50, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -50, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, 50, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -50, true)?;
    sleep(Duration::from_secs(2));*/

    let left_color_sensor = ev3dev_lang_rust::sensors::ColorSensor::get(SensorPort::In1)?;

    for _ in 0..10 {
        left_color_sensor.set_mode_ref_raw()?;
        let result = left_color_sensor.get_color()?;
        println!("{result}");
    }

    println!("done driving");

    drivebase.set_brake_mode(BrakeMode::Coast)?;

    Ok(())
}
