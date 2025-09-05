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
    /// robot.drive(200, Some(500))?;
    /// ```
    /// or
    /// ```rust
    /// robot.drive(200, 500)?;
    /// ```
    ///
    /// Drive forward indefinitely at speed 150:
    /// ```rust
    /// robot.drive(150, None)?;
    /// ```
    #[expect(clippy::cast_possible_truncation)]
    pub fn drive(
        &self,
        speed: i32,
        distance: impl Into<Option<i32>>,
        stop: bool,
    ) -> Result<&Self, Ev3Error> {
        let distance = distance.into();
        self.set_speed(speed, distance)?;

        let Some(distance) = distance else {
            self.run_forever()?;
            return Ok(self);
        };

        let mut left_counts = ((f64::from(distance) / self.circumference)
            * f64::from(self.left.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;
        let mut right_counts = ((f64::from(distance) / self.circumference)
            * f64::from(self.right.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        match self.left_meta.direction {
            Direction::Clockwise => {}
            Direction::CounterClockwise => left_counts *= -1,
        }
        match self.right_meta.direction {
            Direction::Clockwise => {}
            Direction::CounterClockwise => right_counts *= -1,
        }

        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;

        self.wait_until_not_moving(None);

        if stop {
            return Ok(self);
        }

        self.run_forever()?;

        Ok(self)
    }
}
