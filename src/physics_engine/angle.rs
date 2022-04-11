use std::f32::consts::PI;

struct Angle {
    radian: f32,
}

impl Angle {
    pub fn get_radian(&mut self) -> f32 {
        while !(0.0..=2.0 * PI).contains(&self.radian) {
            if self.radian > 2.0 * PI {
                self.radian -= 2.0 * PI;
            } else {
                self.radian += 2.0 * PI;
            }
        }

        self.radian
    }

    pub fn set_radian(&mut self, angle: f32) {
        let mut filtered_angle = angle;

        while !(0.0..=2.0 * PI).contains(&filtered_angle) {
            if filtered_angle > 2.0 * PI {
                filtered_angle -= 2.0 * PI;
            } else {
                filtered_angle += 2.0 * PI;
            }
        }

        self.radian = filtered_angle
    }
}
