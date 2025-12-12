use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Sets the acceleration rate for both motors.
    ///
    /// Acceleration determines how quickly the motors ramp up to the target speed.
    /// Higher values result in faster acceleration but may cause wheel slipping or
    /// mechanical stress. Lower values provide smoother, more controlled starts.
    ///
    /// # Parameters
    ///
    /// - `acceleration`: The acceleration rate in degrees per second squared (deg/s²)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The acceleration value is negative
    /// - The value cannot be set on either motor
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
    ///     // Set moderate acceleration for smooth starts
    ///     drivebase.set_acceleration(2000)?;
    ///
    ///     // Set high acceleration for quick response
    ///     drivebase.set_acceleration(5000)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_acceleration(&self, acceleration: i32) -> Result<&Self, Ev3Error> {
        self.left.set_ramp_up_sp(acceleration)?;
        self.right.set_ramp_up_sp(acceleration)?;
        Ok(self)
    }

    /// Sets the deceleration rate for both motors.
    ///
    /// Deceleration determines how quickly the motors slow down when stopping.
    /// Higher values result in faster stops but may cause the robot to jerk or tip.
    /// Lower values provide smoother, more controlled stops.
    ///
    /// # Parameters
    ///
    /// - `deceleration`: The deceleration rate in degrees per second squared (deg/s²)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The deceleration value is negative
    /// - The value cannot be set on either motor
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
    ///     // Set moderate deceleration for smooth stops
    ///     drivebase.set_deceleration(2000)?;
    ///
    ///     // Set high deceleration for quick stops
    ///     drivebase.set_deceleration(5000)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// Deceleration works in combination with the brake mode:
    /// - With `BrakeMode::Coast`: Deceleration has limited effect
    /// - With `BrakeMode::Brake` or `BrakeMode::Hold`: Full effect
    pub fn set_deceleration(&self, deceleration: i32) -> Result<&Self, Ev3Error> {
        self.left.set_ramp_down_sp(deceleration)?;
        self.right.set_ramp_down_sp(deceleration)?;
        Ok(self)
    }
}
