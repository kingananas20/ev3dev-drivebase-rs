use super::DriveBase;
use crate::Direction;
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

        radius.map_or_else(
            || self.turn_in_place(speed, degree),
            |radius| self.turn_with_radius(speed, degree, radius),
        )
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

    #[expect(clippy::cast_possible_truncation)]
    fn turn_with_radius(&self, speed: i32, degree: i32, radius: f64) -> Result<&Self, Ev3Error> {
        if degree == 0 {
            return Ok(self);
        }

        // scheissendreck funktioniert nicht.
        // when speed > 0, turn left, else right
        // when radius > 0, turn forward, else backward

        let angle_rad = f64::from(degree).to_radians();
        let r = radius.abs();
        let half_track = self.axle_track / 2.;

        // inner and outer wheel radii
        let (inner_radius, outer_radius) = if degree > 0 {
            (r - half_track, r + half_track) // left turn
        } else {
            (r + half_track, r - half_track) // right turn
        };

        // arc lengths for wheels
        let inner_mm = (angle_rad * inner_radius)
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;
        let outer_mm = (angle_rad * outer_radius)
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        // convert to counts
        let (inner_counts, outer_counts) = self.calculate_counts(inner_mm, outer_mm)?;

        // scale inner wheel speed proportionally; outer wheel uses the requested speed
        let outer_speed = speed;
        let inner_speed = ((inner_radius / outer_radius) * f64::from(speed.abs())).round() as i32;

        println!("{inner_speed} {outer_speed} {inner_counts} {outer_counts}");

        /*println!("{left_speed} {right_speed} {left_counts} {right_counts}");

        self.set_speed(left_speed, right_speed)?;

        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;

        self.wait_until_not_moving(None);*/

        Ok(self)
    }
}
