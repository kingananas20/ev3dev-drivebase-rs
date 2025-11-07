use std::fs;

use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Drives the robot forward or backward at the specified speed.
    ///
    /// It's not accurate if the brake mode is set to `Coast`.
    ///
    /// # Parameters
    ///
    /// - `speed`: The speed at which to drive the robot, in tacho counts per second (positive for forward, negative for backward).
    /// - `distance`: Optional distance to drive, in millimeters.
    ///     - If `Some(mm)`, the robot drives that distance and then stops.
    ///     - If `None`, the robot will drive indefinitely until another command stops it.
    ///
    /// # Errors
    ///
    /// Returns an `Ev3Error` if the motor cannot be started or a command fails.
    ///
    /// # Example
    ///
    /// Drive forward 500 mm at speed 200:
    /// ```rust
    /// robot.drive(200, 500)?;
    /// ```
    pub fn drive(
        &self,
        mut speed: i32,
        distance: impl Into<i32>,
        stop: bool,
    ) -> Result<&Self, Ev3Error> {
        let distance: i32 = distance.into();
        if distance == 0 {
            return Ok(self);
        }

        let direction = if distance < 0 {
            Direction::CounterClockwise
        } else {
            Direction::Clockwise
        };

        speed = speed.abs();

        self.set_speed(speed, speed)?;

        let mut counts = self.calculate_counts(distance, distance)?;

        counts.0 = counts.0.saturating_mul(direction.sign());
        counts.1 = counts.1.saturating_mul(direction.sign());

        println!("{counts:?}");

        let result1 = write_motor("motor0", "position_sp", &format!("{}", counts.0));
        let result2 = write_motor("motor1", "position_sp", &format!("{}", counts.1));
        let result3 = write_motor("motor0", "command", "run-to-rel-pos");
        let result4 = write_motor("motor1", "command", "run-to-rel-pos");

        println!("{result1:?}\n{result2:?}\n{result3:?}\n{result4:?}");
        //self.run_to_rel_pos(counts.0, counts.1)?;
        self.wait_until_not_moving(None);

        if !stop {
            self.run_forever()?;
        }

        Ok(self)
    }
}

fn write_motor(motor: &str, file: &str, value: &str) -> std::io::Result<()> {
    fs::write(format!("/sys/class/tacho-motor/{motor}/{file}"), value)
}

fn read_motor(motor: &str, file: &str) -> std::io::Result<String> {
    fs::read_to_string(format!("/sys/class/tacho-motor/{motor}/{file}"))
}
