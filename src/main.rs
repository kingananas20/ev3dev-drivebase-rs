//! Simple program purely for testing

use ev3_drivebase::{
    BrakeMode, Direction, DriveBase, Motor,
    ev3dev_lang_rust::{Ev3Error, motors::MotorPort},
};
use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::Clockwise);
    let right = Motor::new(MotorPort::OutC, Direction::CounterClockwise);
    let mut drivebase = DriveBase::new(left, right, 43.2, 185.)?;
    drivebase
        .set_brake_mode(BrakeMode::Hold)?
        .set_acceleration(500)?
        .set_deceleration(500)?;

    let left_sensor = ColorSensor::get(SensorPort::In1)?;
    let right_sensor = ColorSensor::get(SensorPort::In2)?;

    drivebase.add_colorsensor(left_sensor, right_sensor);

    println!("Drivebase initialisiert");

    use std::{thread::sleep, time::Duration};
    drivebase.turn(500, 90, None)?;
    sleep(Duration::from_secs(2));
    drivebase.turn(500, -90, None)?;

    /*use std::{thread::sleep, time::Duration};
    drivebase.drive(500, 50 * 2, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -50 * 2, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, 50 * 2, true)?;
    sleep(Duration::from_secs(2));
    drivebase.drive(500, -50 * 2, true)?;
    sleep(Duration::from_secs(2));*/

    /*let left_color_sensor = ev3dev_lang_rust::sensors::ColorSensor::get(SensorPort::In1)?;

    for _ in 0..10 {
        left_color_sensor.set_mode_ref_raw()?;
        let result = left_color_sensor.get_color()?;
        println!("{result}");
    }*/

    println!("done driving");

    drivebase.set_brake_mode(BrakeMode::Coast)?;

    Ok(())
}
