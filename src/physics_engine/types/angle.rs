use std::f32::consts::PI;

/// Angle structure
#[derive(Debug, Copy, Clone)]
pub struct Angle {
    radian: f32,
}

impl Angle {
    /// Creating a angle
    pub fn new(radian: f32) -> Self {
        let mut temp_radian = radian;
        while !(0.0..=2.0 * PI).contains(&temp_radian) {
            if temp_radian > 2.0 * PI {
                temp_radian -= 2.0 * PI;
            } else {
                temp_radian += 2.0 * PI;
            }
        }

        Angle {
            radian: temp_radian,
        }
    }

    pub fn get_radian(&self) -> f32 {
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


impl Default for Angle {
    fn default() -> Self {
        Angle::new(0.0)
    }
}
