use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Drives the robot forward or backward for a specified distance.
    ///
    /// The robot will drive in a straight line at the given speed until it has traveled
    /// the specified distance, then stop according to the configured brake mode.
    ///
    /// # Parameters
    ///
    /// - `speed`: The speed in degrees per second. Positive values use forward direction,
    ///   negative values reverse the direction.
    /// - `distance`: The distance to travel in millimeters. Positive for forward, negative for backward.
    /// - `stop`: Whether to stop the motors after reaching the target distance.
    ///   - `true`: Motors will stop when the distance is reached
    ///   - `false`: Motors will continue running at the same speed after reaching the distance
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The motors cannot be commanded
    /// - Speed or distance calculations fail
    ///
    /// # Examples
    ///
    /// Basic forward and backward movement:
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///     drivebase.set_brake_mode(BrakeMode::Hold)?;
    ///
    ///     // Drive forward 300mm at 200 deg/s and stop
    ///     drivebase.drive(200, 300, true)?;
    ///
    ///     // Drive backward 150mm at 100 deg/s and stop
    ///     drivebase.drive(100, -150, true)?;
    ///
    ///     // Drive forward 500mm and continue running
    ///     drivebase.drive(300, 500, false)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Behavior
    ///
    /// - If `distance` is 0, the method returns immediately without moving
    /// - The direction of travel is determined by the sign of `distance`
    /// - The method blocks until the robot reaches the target distance
    /// - Motor speeds are automatically adjusted based on the configured motor directions
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
