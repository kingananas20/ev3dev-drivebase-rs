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

        self.set_speed(speed, direction)?;

        let distance_abs = distance.abs();

        let mut left_counts = ((f64::from(distance_abs) / self.circumference)
            * f64::from(self.left.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        let mut right_counts = ((f64::from(distance_abs) / self.circumference)
            * f64::from(self.right.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        println!("1 {left_counts} {right_counts}");

        left_counts = left_counts
            .saturating_mul(self.left_meta.direction.sign())
            .saturating_mul(direction.sign());
        right_counts = right_counts
            .saturating_mul(self.right_meta.direction.sign())
            .saturating_mul(direction.sign());

        println!("2 {left_counts} {right_counts}");

        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;
        self.wait_until_not_moving(None);

        if !stop {
            self.run_forever()?;
        }

        Ok(self)
    }
}
