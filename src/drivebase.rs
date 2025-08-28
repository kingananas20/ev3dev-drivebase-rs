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
}

impl Drop for DriveBase {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
