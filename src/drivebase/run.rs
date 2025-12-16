use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    /// Runs the robot forever using the already set speed
    ///
    /// # Errors
    ///
    /// Errors if it can run the command to run the robot forever
    pub fn run_forever(&self) -> Result<&Self, Ev3Error> {
        self.left.run_forever()?;
        self.right.run_forever()?;
        Ok(self)
    }

    /// Runs the robot to a relative position using the already set speed
    ///
    /// # Errors
    ///
    /// Errors if it can't run the command
    pub fn run_to_rel_pos(
        &self,
        left_position: i32,
        right_position: i32,
    ) -> Result<&Self, Ev3Error> {
        self.left.run_to_rel_pos(Some(left_position))?;
        self.right.run_to_rel_pos(Some(right_position))?;
        Ok(self)
    }
}
