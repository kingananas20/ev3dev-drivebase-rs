use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Sets the acceleration of the drivebase in deg/s^2
    ///
    /// # Errors
    ///
    /// Errors if the value is negative or it can't set the value
    pub fn set_acceleration(&self, acceleration: i32) -> Result<&Self, Ev3Error> {
        self.left.set_ramp_up_sp(acceleration)?;
        self.right.set_ramp_up_sp(acceleration)?;
        Ok(self)
    }

    /// Sets the deceleration of the drivebase in deg/s^2
    ///
    /// # Errors
    ///
    /// Errors if the value is negative or it can't set the value
    pub fn set_deceleration(&self, deceleration: i32) -> Result<&Self, Ev3Error> {
        self.left.set_ramp_down_sp(deceleration)?;
        self.right.set_ramp_down_sp(deceleration)?;
        Ok(self)
    }
}
