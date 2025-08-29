use crate::Direction;

use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    pub(super) fn set_speed(&self, speed: i32) -> Result<(), Ev3Error> {
        let speed_left = match self.left_meta.direction {
            Direction::Clockwise => speed,
            Direction::CounterClockwise => -speed,
        };
        let speed_right = match self.right_meta.direction {
            Direction::Clockwise => speed,
            Direction::CounterClockwise => -speed,
        };

        self.left.set_speed_sp(speed_left)?;
        self.right.set_speed_sp(speed_right)?;
        Ok(())
    }
}
