use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Drives the robot forward or backward at the specified speed.
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
        &mut self,
        speed: i32,
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

        self.set_speed(speed, speed)?;

        let mut counts = self.calculate_counts(distance, distance)?;

        counts.0 = counts.0.saturating_mul(direction.sign());
        counts.1 = counts.1.saturating_mul(direction.sign());

        self.run_to_rel_pos(counts.0, counts.1)?;
        self.wait_until_not_moving(None);

        if !stop {
            self.run_forever()?;
        }

        Ok(self)
    }
}
