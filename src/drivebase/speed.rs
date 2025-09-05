use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;

impl DriveBase {
    #[expect(clippy::cast_possible_truncation)]
    pub(super) fn set_speed(&self, speed: i32, distance: Option<i32>) -> Result<(), Ev3Error> {
        let speed_left = match self.left_meta.direction {
            Direction::Clockwise => speed,
            Direction::CounterClockwise => -speed,
        };
        let speed_right = match self.right_meta.direction {
            Direction::Clockwise => speed,
            Direction::CounterClockwise => -speed,
        };

        let (ramp_up_ms, ramp_down_ms) = distance.map_or((500, 500), |dist| {
            let ramp_fraction = 0.1f64;
            let min_ramp = 50;
            let max_ramp = 1000;

            let ramp_time = ((f64::from(dist) * ramp_fraction) * 10.0)
                .round()
                .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;
            let ramp_time = ramp_time.clamp(min_ramp, max_ramp);

            (ramp_time, ramp_time)
        });

        self.left.set_ramp_up_sp(ramp_up_ms)?;
        self.left.set_ramp_down_sp(ramp_down_ms)?;
        self.right.set_ramp_up_sp(ramp_up_ms)?;
        self.right.set_ramp_down_sp(ramp_down_ms)?;

        self.left.set_speed_sp(speed_left)?;
        self.right.set_speed_sp(speed_right)?;
        Ok(())
    }
}
