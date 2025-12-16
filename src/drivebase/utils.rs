//! Utility functions for drivebase control and status checking.

use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Stops both motors immediately.
    ///
    /// The motors will stop according to the configured brake mode:
    /// - `Coast`: Motors freely coast to a stop
    /// - `Brake`: Motors actively brake
    /// - `Hold`: Motors hold their current position
    ///
    /// This method is automatically called when the `DriveBase` is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the stop command cannot be sent to either motor.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     // Start driving
    ///     drivebase.drive(300, 1000, false)?;
    ///
    ///     // Stop immediately
    ///     drivebase.stop()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn stop(&self) -> Result<&Self, Ev3Error> {
        self.left.stop()?;
        self.right.stop()?;
        Ok(self)
    }

    /// Resets all motor parameters to their default values.
    ///
    /// This method:
    /// - Stops both motors
    /// - Resets position counters to zero
    /// - Clears any pending commands
    /// - Resets speed, acceleration, and other settings to defaults
    ///
    /// This is automatically called when creating a new `DriveBase`.
    ///
    /// # Errors
    ///
    /// Returns an error if the reset command fails on either motor.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     // After some movements, reset everything
    ///     drivebase.reset()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn reset(&self) -> Result<&Self, Ev3Error> {
        self.left.reset()?;
        self.right.reset()?;
        Ok(self)
    }

    /// Checks if power is being sent to either motor.
    ///
    /// Returns `true` if at least one motor is currently running (receiving power),
    /// even if the motor is ramping up or down.
    ///
    /// # Errors
    ///
    /// Returns an error if the motor state cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     drivebase.drive(200, 500, false)?;
    ///
    ///     if drivebase.is_running()? {
    ///         println!("Motors are running!");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn is_running(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_running()? || self.right.is_running()?)
    }

    /// Checks if either motor is currently ramping up to speed.
    ///
    /// Returns `true` during the acceleration phase after a motor command is issued,
    /// before the motor reaches its target speed.
    ///
    /// # Errors
    ///
    /// Returns an error if the motor state cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     drivebase.drive(300, 1000, false)?;
    ///
    ///     if drivebase.is_ramping()? {
    ///         println!("Motors are accelerating!");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn is_ramping(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_ramping()? || self.right.is_ramping()?)
    }

    /// Checks if either motor is actively holding its position.
    ///
    /// Returns `true` when a motor has stopped and is in `BrakeMode::Hold`,
    /// actively maintaining its position against external forces.
    ///
    /// # Errors
    ///
    /// Returns an error if the motor state cannot be read.
    ///
    /// # Examples
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
    ///     drivebase.drive(200, 300, true)?;
    ///
    ///     if drivebase.is_holding()? {
    ///         println!("Motors are holding position!");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn is_holding(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_holding()? || self.right.is_holding()?)
    }

    /// Checks if either motor is overloaded.
    ///
    /// Returns `true` if a motor is drawing excessive current, which typically indicates:
    /// - The robot is stuck against an obstacle
    /// - The motor is trying to move a load that's too heavy
    /// - Mechanical binding or damage
    ///
    /// # Errors
    ///
    /// Returns an error if the motor state cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     drivebase.drive(300, 1000, false)?;
    ///
    ///     if drivebase.is_overloaded()? {
    ///         println!("Warning: Motors overloaded!");
    ///         drivebase.stop()?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn is_overloaded(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_overloaded()? || self.right.is_overloaded()?)
    }

    /// Checks if either motor has stalled.
    ///
    /// Returns `true` if a motor is not moving despite receiving power, indicating:
    /// - The robot is blocked by an obstacle
    /// - The wheels are slipping
    /// - Mechanical failure
    ///
    /// # Errors
    ///
    /// Returns an error if the motor state cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3_drivebase::{DriveBase, Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};
    ///
    /// fn main() -> Result<(), Ev3Error> {
    ///     let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    ///     let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    ///     let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     drivebase.drive(300, 1000, false)?;
    ///
    ///     if drivebase.is_stalled()? {
    ///         println!("Warning: Motors stalled!");
    ///         drivebase.stop()?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn is_stalled(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_stalled()? || self.right.is_stalled()?)
    }

    /// Calculates encoder counts for a given distance in millimeters.
    ///
    /// This internal method converts distance in millimeters to motor encoder counts,
    /// taking into account wheel circumference and motor direction.
    ///
    /// # Parameters
    ///
    /// - `left_distance`: Distance for the left wheel in millimeters
    /// - `right_distance`: Distance for the right wheel in millimeters
    ///
    /// # Returns
    ///
    /// A tuple `(left_counts, right_counts)` representing the encoder counts for each motor,
    /// with correct sign according to motor direction.
    ///
    /// # Errors
    ///
    /// Returns an error if encoder information cannot be read from the motors.
    #[expect(clippy::cast_possible_truncation)]
    pub fn calculate_counts(
        &self,
        left_distance: i32,
        right_distance: i32,
    ) -> Result<(i32, i32), Ev3Error> {
        let left_distance = left_distance.abs();
        let right_distance = right_distance.abs();

        let mut left_counts = ((f64::from(left_distance) / self.circumference)
            * f64::from(self.left.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        let mut right_counts = ((f64::from(right_distance) / self.circumference)
            * f64::from(self.right.get_count_per_rot()?))
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        left_counts = left_counts.saturating_mul(self.left_meta.direction.sign());
        right_counts = right_counts.saturating_mul(self.right_meta.direction.sign());

        Ok((left_counts, right_counts))
    }
}

impl Drop for DriveBase {
    fn drop(&mut self) {
        let _ = self.stop();
        let _ = self.set_brake_mode(super::BrakeMode::Coast);
    }
}
