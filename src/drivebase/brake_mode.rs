use super::DriveBase;
use ev3dev_lang_rust::{Ev3Error, motors::TachoMotor};

impl DriveBase {
    /// Sets the brake mode of both motors. Its either `Coast`, `Brake` or `Hold`.
    ///
    /// # Errors
    ///
    /// Errors if it can't set the command
    pub fn set_brake_mode(&self, brake_mode: BrakeMode) -> Result<&Self, Ev3Error> {
        self.left.set_stop_action(brake_mode.as_ref())?;
        self.right.set_stop_action(brake_mode.as_ref())?;
        Ok(self)
    }

    /// Gets the brake mode for both motors. If the motors have different brake modes, this sets the right motor's
    /// brake mode to match the left motor's.
    ///
    /// # Errors
    ///
    /// Errors if it can't read the brake mode
    pub fn get_brake_mode(&self) -> Result<BrakeMode, Ev3Error> {
        let left_brake_mode: BrakeMode = self.left.get_stop_action()?.into();
        let right_brake_mode: BrakeMode = self.right.get_stop_action()?.into();

        if right_brake_mode != left_brake_mode {
            self.right.set_stop_action(left_brake_mode.as_ref())?;
        }

        Ok(left_brake_mode)
    }
}

/// The brake mode of the motor
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrakeMode {
    /// The motor will freely coast to a stop. No active braking is applied.
    Coast,
    /// The motor will actively brake, resisting motion until it comes to a stop.
    /// Slows the motor faster than `Coast` but does not hold position.
    Brake,
    /// The motor will actively hold its current position once stopped.
    /// Useful for precise positioning, but can consume more power.
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
