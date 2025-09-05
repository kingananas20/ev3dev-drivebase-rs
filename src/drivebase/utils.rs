use std::time::Duration;

use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Stops both motors and is called on drop.
    ///
    /// # Errors
    ///
    /// Errors if it can't set the stop command.
    pub fn stop(&self) -> Result<&Self, Ev3Error> {
        self.left.stop()?;
        self.right.stop()?;
        Ok(self)
    }

    /// Resets all of the parameters of both motors and the `DriveBase`. Has the added effect of stopping.
    ///
    /// # Errors
    ///
    /// Errors if it can't reset the `DriveBase`.
    pub fn reset(&self) -> Result<&Self, Ev3Error> {
        self.left.reset()?;
        self.right.reset()?;
        Ok(self)
    }

    pub(super) fn wait_until_not_moving(&self, timeout: Option<Duration>) -> &Self {
        self.left.wait_until_not_moving(timeout);
        self.right.wait_until_not_moving(timeout);
        self
    }

    pub(super) fn run_to_rel_pos(
        &self,
        left_position: Option<i32>,
        right_position: Option<i32>,
    ) -> Result<&Self, Ev3Error> {
        self.left.run_to_rel_pos(left_position)?;
        self.right.run_to_rel_pos(right_position)?;
        Ok(self)
    }

    /// If power is being sent to both motors.
    ///
    /// # Errors
    ///
    /// Errors if it can't set the stop command.
    pub fn is_running(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_running()? || self.right.is_running()?)
    }

    /// If any one of the motors are still ramping up.
    ///
    /// # Errors
    ///
    /// Errors if it can't read the sysfs
    pub fn is_ramping(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_ramping()? || self.right.is_ramping()?)
    }

    /// If any one of the motors are holding.
    ///
    /// # Errors
    ///
    /// Errors if it can't read the sysfs
    pub fn is_holding(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_holding()? && self.right.is_holding()?)
    }

    /// If any one of the motors are overloaded.
    ///
    /// # Errors
    ///
    /// Errors if it can't read the sysfs
    pub fn is_overloaded(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_overloaded()? || self.right.is_overloaded()?)
    }

    /// If any one of the motors are stalled.
    ///
    /// # Errors
    ///
    /// Errors if it can't read the sysfs
    pub fn is_stalled(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_stalled()? || self.right.is_stalled()?)
    }
}

impl Drop for DriveBase {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
