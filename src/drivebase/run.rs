use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    pub(super) fn run_forever(&self) -> Result<&Self, Ev3Error> {
        self.left.run_forever()?;
        self.right.run_forever()?;
        Ok(self)
    }

    pub(super) fn run_to_rel_pos(
        &self,
        left_position: i32,
        right_position: i32,
    ) -> Result<&Self, Ev3Error> {
        self.left.run_to_rel_pos(Some(left_position))?;
        self.right.run_to_rel_pos(Some(right_position))?;
        Ok(self)
    }
}
