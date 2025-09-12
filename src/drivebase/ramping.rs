use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Sets the acceleration of the drivebase in deg/s^2
    ///
    /// # Errors
    ///
    /// Errors if the value is negative or it can't set the value
    pub fn set_acceleration(&self, acceleration: i32) -> Result<&Self, Ev3Error> {
        let acceleration = self.calculate_time(acceleration)?;
        self.left.set_ramp_up_sp(acceleration.0)?;
        self.right.set_ramp_up_sp(acceleration.1)?;
        Ok(self)
    }

    /// Sets the deceleration of the drivebase in deg/s^2
    ///
    /// # Errors
    ///
    /// Errors if the value is negative or it can't set the value
    pub fn set_deceleration(&self, deceleration: i32) -> Result<&Self, Ev3Error> {
        let deceleration = self.calculate_time(deceleration)?;
        self.left.set_ramp_down_sp(deceleration.0)?;
        self.right.set_ramp_down_sp(deceleration.1)?;
        Ok(self)
    }

    fn calculate_time(&self, target: i32) -> Result<(i32, i32), Ev3Error> {
        let left_max_speed = self.left.get_max_speed()?;
        let right_max_speed = self.right.get_max_speed()?;

        let left_ramp_time = left_max_speed / target * 1000;
        let right_ramp_time = right_max_speed / 1000;

        Ok((left_ramp_time, right_ramp_time))
    }
}
