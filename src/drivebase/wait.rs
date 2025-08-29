#![expect(unused)]

use super::DriveBase;
use core::time::Duration;
use ev3dev_lang_rust::motors::TachoMotor;

impl DriveBase {
    /// Wait until condition cond returns true or the timeout is reached.
    ///
    /// The condition is checked when to the state attribute has changed. If the timeout is None it will wait an infinite time.
    pub fn wait<F>(&self, cond: F, timeout: Option<Duration>)
    where
        F: Fn(&TachoMotor, &TachoMotor) -> bool + Send + Sync,
    {
    }
}
