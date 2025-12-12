use super::DriveBase;
use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor};

impl DriveBase {
    /// Sets the brake mode for both motors.
    ///
    /// The brake mode determines how the motors behave when stopped:
    ///
    /// - **Coast**: Motors freely coast to a stop (no braking)
    /// - **Brake**: Motors actively brake but don't hold position
    /// - **Hold**: Motors hold their current position with power
    ///
    /// # Parameters
    ///
    /// - `brake_mode`: The desired brake mode to apply to both motors
    ///
    /// # Errors
    ///
    /// Returns an error if the brake mode cannot be set on either motor.
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
    ///     let drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     // Use Hold mode for precise positioning
    ///     drivebase.set_brake_mode(BrakeMode::Hold)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_brake_mode(&self, brake_mode: BrakeMode) -> Result<&Self, Ev3Error> {
        self.left.set_stop_action(brake_mode.as_ref())?;
        self.right.set_stop_action(brake_mode.as_ref())?;
        Ok(self)
    }

    /// Gets the current brake mode for both motors.
    ///
    /// If the motors have different brake modes, this method will set the right motor's
    /// brake mode to match the left motor's and return the left motor's mode.
    ///
    /// # Errors
    ///
    /// Returns an error if the brake mode cannot be read or set.
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
    ///     let drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    ///
    ///     let current_mode = drivebase.get_brake_mode()?;
    ///     println!("Current brake mode: {:?}", current_mode);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_brake_mode(&self) -> Result<BrakeMode, Ev3Error> {
        let left_brake_mode: BrakeMode = self.left.get_stop_action()?.into();
        let right_brake_mode: BrakeMode = self.right.get_stop_action()?.into();

        if right_brake_mode != left_brake_mode {
            self.right.set_stop_action(left_brake_mode.as_ref())?;
        }

        Ok(left_brake_mode)
    }
}

/// The brake mode determines how motors behave when stopped.
///
/// Each mode offers different trade-offs between stopping speed, position accuracy,
/// and power consumption:
///
/// - **Coast**: Lowest power consumption, but least precise stopping
/// - **Brake**: Good balance between speed and precision
/// - **Hold**: Most precise, but highest power consumption
///
/// # Examples
///
/// ```
/// use ev3_drivebase::BrakeMode;
///
/// // For general movement where exact stopping position doesn't matter
/// let casual = BrakeMode::Coast;
///
/// // For controlled stopping without position holding
/// let controlled = BrakeMode::Brake;
///
/// // For precise positioning tasks
/// let precise = BrakeMode::Hold;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrakeMode {
    /// The motor will freely coast to a stop.
    ///
    /// No active braking is applied. The motor will gradually slow down due to friction.
    /// This mode uses the least power but provides the least control over stopping position.
    Coast,

    /// The motor will actively brake, resisting motion until it comes to a stop.
    ///
    /// Slows the motor faster than `Coast` but does not hold the final position.
    /// After stopping, the motor can be moved freely by external forces.
    Brake,

    /// The motor will actively hold its current position once stopped.
    ///
    /// Provides the most precise stopping and maintains position against external forces.
    /// This mode consumes more power as it continuously applies holding torque.
    Hold,
}

impl AsRef<str> for BrakeMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Coast => TachoMotor::STOP_ACTION_COAST,
            Self::Brake => TachoMotor::STOP_ACTION_BRAKE,
            Self::Hold => TachoMotor::STOP_ACTION_HOLD,
        }
    }
}

impl From<&str> for BrakeMode {
    fn from(value: &str) -> Self {
        match value {
            TachoMotor::STOP_ACTION_BRAKE => Self::Brake,
            TachoMotor::STOP_ACTION_HOLD => Self::Hold,
            _ => Self::Coast,
        }
    }
}

impl From<String> for BrakeMode {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
