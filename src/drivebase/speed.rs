use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    pub(super) fn set_speed(&self, speed: i32, direction: Direction) -> Result<(), Ev3Error> {
        let speed_left = speed
            .saturating_mul(self.left_meta.direction.sign())
            .saturating_mul(direction.sign());
        let speed_right = speed
            .saturating_mul(self.right_meta.direction.sign())
            .saturating_mul(direction.sign());

        /*self.left.set_ramp_up_sp(ramp_up_ms)?;
        self.left.set_ramp_down_sp(ramp_down_ms)?;
        self.right.set_ramp_up_sp(ramp_up_ms)?;
        self.right.set_ramp_down_sp(ramp_down_ms)?;*/

        self.left.set_speed_sp(speed_left)?;
        self.right.set_speed_sp(speed_right)?;
        Ok(())
    }
}
