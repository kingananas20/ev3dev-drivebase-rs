use ev3dev_lang_rust::{
    Ev3Error,
    motors::{MotorPort, TachoMotor},
};

pub struct DriveBase {
    pub left: TachoMotor,
    pub right: TachoMotor,
}

impl DriveBase {
    pub fn new(left: MotorPort, right: MotorPort) -> Result<Self, Ev3Error> {
        let left = TachoMotor::get(left)?;
        let right = TachoMotor::get(right)?;
        Ok(Self { left, right })
    }

    pub fn stop(&self) -> Result<&Self, Ev3Error> {
        self.left.stop()?;
        self.right.stop()?;
        Ok(self)
    }

    pub fn reset(&self) -> Result<&Self, Ev3Error> {
        self.left.reset()?;
        self.right.reset()?;
        Ok(self)
    }

    pub fn is_running(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_running()? && self.right.is_running()?)
    }

    pub fn is_ramping(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_ramping()? && self.right.is_ramping()?)
    }

    pub fn is_holding(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_holding()? && self.right.is_holding()?)
    }

    pub fn is_overloaded(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_overloaded()? && self.right.is_overloaded()?)
    }

    pub fn is_stalled(&self) -> Result<bool, Ev3Error> {
        Ok(self.left.is_stalled()? && self.right.is_stalled()?)
    }
}

impl Drop for DriveBase {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
