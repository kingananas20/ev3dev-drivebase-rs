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
        let mut radius: Option<f64> = radius.into();
        radius = radius.map(f64::abs);

        radius.map_or_else(
            || self.turn_in_place(speed, degree),
            |radius| self.turn_with_radius(speed, degree, radius),
        )
    }

    #[expect(clippy::cast_possible_truncation)]
    fn turn_in_place(&self, speed: i32, degree: i32) -> Result<&Self, Ev3Error> {
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

        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;

        self.wait_until_not_moving(None);

        Ok(self)
    }

    #[expect(clippy::cast_possible_truncation)]
    fn turn_with_radius(&self, speed: i32, degree: i32, radius: f64) -> Result<&Self, Ev3Error> {
        // nothing to do
        if degree == 0 {
            return Ok(self);
        }

        let theta = (f64::from(degree)).to_radians(); // signed angle in radians
        let theta_abs = theta.abs();
        let half_track = self.axle_track / 2.0;
        let eps = 1e-9;

        // helper: sign of a number as i32
        let sign_f64 = |v: f64| if v >= 0.0 { 1 } else { -1 };
        let deg_is_left = degree > 0;
        let s = f64::from(speed);

        // outputs (floating intermediate)
        let (left_speed_f, right_speed_f, left_arc_mm, right_arc_mm) = if radius.abs() < eps {
            // in-place spin: each wheel travels arc = (axle_track / 2) * |theta|
            let arc = half_track * theta_abs;
            // for an in-place left turn (degree>0): left wheel goes backward, right forward.
            // We encode that by multiplying speed by sign(degree) with a flip on the left wheel.
            let deg_sign = if deg_is_left { 1.0 } else { -1.0 };
            let left_speed = -s * deg_sign;
            let right_speed = s * deg_sign;
            (left_speed, right_speed, arc, arc)
        } else {
            // non-zero radius: compute left/right path radii relative to robot center.
            // radius parameter is magnitude; use `degree` to decide which side is inner.
            let r = radius.abs();
            let (r_left, r_right) = if deg_is_left {
                (r - half_track, r + half_track)
            } else {
                (r + half_track, r - half_track)
            };

            // wheel linear speeds scale with their path radii: v_wheel = v_center * (r_wheel / r)
            // if r_left is negative the wheel will run opposite sign (path center inside wheel).
            let left_speed = s * (r_left / r);
            let right_speed = s * (r_right / r);

            let left_arc = theta_abs * r_left.abs();
            let right_arc = theta_abs * r_right.abs();
            (left_speed, right_speed, left_arc, right_arc)
        };

        // convert arcs -> wheel rotations -> motor tacho degrees
        // rotations = arc_mm / circumference_mm
        let left_rot = left_arc_mm / self.circumference;
        let right_rot = right_arc_mm / self.circumference;

        // tacho degrees needed by each motor (wheel rotations * 360)
        let left_tacho_deg = (left_rot * 360.0).round();
        let right_tacho_deg = (right_rot * 360.0).round();

        // Map computed wheel speeds into motor speed units and motor direction sign.
        // Multiply by motor metadata direction so the resulting command sign is correct for the physical motor.
        let left_speed =
            (left_speed_f.round() as i32).saturating_mul(self.left_meta.direction.sign());
        let right_speed =
            (right_speed_f.round() as i32).saturating_mul(self.right_meta.direction.sign());

        // Counts should carry sign depending on the commanded motor rotation (speed direction)
        let left_counts = (left_tacho_deg as i32)
            .saturating_mul(sign_f64(left_speed_f) * self.left_meta.direction.sign());
        let right_counts = (right_tacho_deg as i32)
            .saturating_mul(sign_f64(right_speed_f) * self.right_meta.direction.sign());

        println!("{left_speed}/{right_speed}  {left_counts}/{right_counts}");

        self.set_speed(left_speed, right_speed)?;
        self.run_to_rel_pos(Some(left_counts), Some(right_counts))?;
        self.wait_until_not_moving(None);

        Ok(self)
    }
}
