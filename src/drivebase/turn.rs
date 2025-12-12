use super::DriveBase;
use crate::Direction;
use ev3dev_lang_rust::Ev3Error;
use std::f64::consts::PI;

impl DriveBase {
    #[expect(clippy::missing_errors_doc, missing_docs)]
    pub fn turn(
        &mut self,
        speed: i32,
        degree: i32,
        radius: impl Into<Option<f64>>,
    ) -> Result<&mut Self, Ev3Error> {
        let mut radius: Option<f64> = radius.into();
        radius = radius.map(f64::abs);

        if degree == 0 {
            return Ok(self);
        }

        match radius {
            None => self.turn_in_place(speed, degree),
            Some(radius) => self.turn_with_radius(speed, degree, radius),
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    fn turn_in_place(&mut self, speed: i32, degree: i32) -> Result<&mut Self, Ev3Error> {
        let speed = speed.abs();
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

        self.run_to_rel_pos(left_counts, right_counts)?;

        self.wait_until_not_moving(None);

        Ok(self)
    }

    #[expect(clippy::cast_possible_truncation)]
    fn turn_with_radius(
        &mut self,
        mut speed: i32,
        degree: i32,
        radius: f64,
    ) -> Result<&mut Self, Ev3Error> {
        speed = speed.clamp(
            -self.left.get_max_speed()? / 2,
            self.left.get_max_speed()? / 2,
        );

        let theta = (f64::from(degree)).to_radians();
        let theta_abs = theta.abs();
        let half_track = self.axle_track / 2.0;
        let eps = 1e-9;

        let sign_f64 = |v: f64| if v >= 0.0 { 1 } else { -1 };
        let deg_is_left = degree > 0;
        let s = f64::from(speed);

        let (left_speed_f, right_speed_f, left_arc_mm, right_arc_mm) = if radius.abs() < eps {
            let arc = half_track * theta_abs;
            let deg_sign = if deg_is_left { 1.0 } else { -1.0 };
            let left_speed = -s * deg_sign;
            let right_speed = s * deg_sign;
            (left_speed, right_speed, arc, arc)
        } else {
            let r = radius.abs();
            let (r_left, r_right) = if deg_is_left {
                (r - half_track, r + half_track)
            } else {
                (r + half_track, r - half_track)
            };

            let left_speed = s * (r_left / r);
            let right_speed = s * (r_right / r);

            let left_arc = theta_abs * r_left.abs();
            let right_arc = theta_abs * r_right.abs();
            (left_speed, right_speed, left_arc, right_arc)
        };

        let left_rot = left_arc_mm / self.circumference;
        let right_rot = right_arc_mm / self.circumference;

        let left_tacho_deg = (left_rot * 360.0).round();
        let right_tacho_deg = (right_rot * 360.0).round();

        let left_speed =
            (left_speed_f.round() as i32).saturating_mul(self.left_meta.direction.sign());
        let right_speed =
            (right_speed_f.round() as i32).saturating_mul(self.right_meta.direction.sign());

        let left_counts = (left_tacho_deg as i32)
            .saturating_mul(sign_f64(left_speed_f) * self.left_meta.direction.sign());
        let right_counts = (right_tacho_deg as i32)
            .saturating_mul(sign_f64(right_speed_f) * self.right_meta.direction.sign());

        self.set_speed(left_speed, right_speed)?;
        self.run_to_rel_pos(left_counts, right_counts)?;
        self.wait_until_not_moving(None);

        Ok(self)
    }
}
