use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    pub(super) fn run_forever(&self) -> Result<&Self, Ev3Error> {
        self.left.run_forever()?;
        self.right.run_forever()?;
        Ok(self)
    }
}
