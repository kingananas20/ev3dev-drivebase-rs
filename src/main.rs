use ev3::{DriveBase, ev3dev_lang_rust::motors::MotorPort};
use ev3dev_lang_rust::Ev3Error;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Ev3Error> {
    let drivebase = DriveBase::new(MotorPort::OutC, MotorPort::OutD)?;
    drivebase.reset()?;

    /*left.set_speed_sp(500)?;
    right.set_speed_sp(500)?;

    left.run_forever()?;
    right.run_forever()?;*/

    sleep(Duration::from_secs(10));

    drivebase.stop()?;

    Ok(())
}
