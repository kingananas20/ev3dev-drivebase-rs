use crate::Direction;

use super::DriveBase;
use ev3dev_lang_rust::Ev3Error;
use std::f64::consts::PI;

impl DriveBase {
    #[expect(clippy::missing_errors_doc, missing_docs)]
    pub fn turn(
        &self,
        speed: i32,
        degree: i32,
        radius: impl Into<Option<f64>>,
    ) -> Result<&Self, Ev3Error> {
        let speed = speed.abs();
        let radius: Option<f64> = radius.into();

        match radius {
            Some(_) => Ok(self),
            None => self.turn_in_place(speed, degree),
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    fn turn_in_place(&self, speed: i32, degree: i32) -> Result<&Self, Ev3Error> {
        let axle_radius = self.axle_track / 2.;
        let arc_length = (axle_radius * f64::from(degree) * (PI / 180.))
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        let direction = if degree >= 0 {
            Direction::CounterClockwise
        } else {
            Direction::Clockwise
        };

        let counts = self.calculate_counts(arc_length, arc_length)?;
        let (left_counts, right_counts) = match direction {
            Direction::CounterClockwise => (-counts.0, counts.1),
            Direction::Clockwise => (counts.0, -counts.1),
        };

        self.set_speed(speed, speed)?;

        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;

        self.wait_until_not_moving(None);

        Ok(self)
    }
}
