use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    pub(super) fn set_speed(&self, left_speed: i32, right_speed: i32) -> Result<(), Ev3Error> {
        self.left.set_speed_sp(left_speed)?;
        self.right.set_speed_sp(right_speed)?;
        Ok(())
    }
}
