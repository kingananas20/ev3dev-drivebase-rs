use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Sets the speed of both motors
    ///
    /// # Errors
    ///
    /// Errors if it can't set the speed of either motor
    pub fn set_speed(&mut self, left_speed: i32, right_speed: i32) -> Result<(), Ev3Error> {
        self.current_speed = left_speed.midpoint(right_speed);
        self.left.set_speed_sp(left_speed)?;
        self.right.set_speed_sp(right_speed)?;
        Ok(())
    }
}
