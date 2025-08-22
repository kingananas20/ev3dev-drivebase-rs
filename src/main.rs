use ev3::ev3dev_lang_rust::motors::MotorPort;
use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Ev3Error> {
    let left = TachoMotor::get(MotorPort::OutC)?;
    let right = TachoMotor::get(MotorPort::OutD)?;

    left.set_speed_sp(500)?;
    right.set_speed_sp(500)?;

    left.run_forever()?;
    right.run_forever()?;

    sleep(Duration::from_secs(10));

    left.stop()?;
    right.stop()?;

    Ok(())
}
