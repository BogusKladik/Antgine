use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2D {
    fn default() -> Self {
        Vec2D::new(0.0, 0.0)
    }
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2D { x, y }
    }

    pub fn cross_pointvv(first_vector: [&Vec2D; 2], second_vector: [&Vec2D; 2]) -> Option<Vec2D> {
        let x1 = first_vector[0].x;
        let y1 = first_vector[0].y;
        let x2 = first_vector[1].x;
        let y2 = first_vector[1].y;

        let x3 = second_vector[0].x;
        let y3 = second_vector[0].y;
        let x4 = second_vector[1].x;
        let y4 = second_vector[1].y;

        let parallelism = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        match parallelism {
            parallelism if parallelism == 0.0 => None,
            _ => {
                let x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
                    / parallelism;
                let y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
                    / parallelism;

                if (x1.min(x2) <= x)
                    && (x1.max(x2) >= x)
                    && (x3.min(x4) <= x)
                    && (x3.max(x4) >= x)
                    && (y1.min(y2) <= y)
                    && (y1.max(y2) >= y)
                    && (y3.min(y4) <= y)
                    && (y3.max(y4) >= y)
                {
                    Some(Vec2D::new(x, y))
                } else {
                    None
                }
            }
        }
    }

    pub fn cross_pointll(first_line: [&Vec2D; 2], second_line: [&Vec2D; 2]) -> Option<Vec2D> {
        let x1 = first_line[0].x;
        let y1 = first_line[0].y;
        let x2 = first_line[1].x;
        let y2 = first_line[1].y;

        let x3 = second_line[0].x;
        let y3 = second_line[0].y;
        let x4 = second_line[1].x;
        let y4 = second_line[1].y;

        let parallelism = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        match parallelism {
            parallelism if parallelism == 0.0 => None,
            _ => {
                let x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
                    / parallelism;
                let y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
                    / parallelism;

                Some(Vec2D::new(x, y))
            }
        }
    }

    pub fn len_vector(&self, point: &Vec2D) -> f32 {
        ((self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)).powf(0.5)
    }

    pub fn unit(&self) -> Vec2D {
        let point_zero = Vec2D::new(0.0, 0.0);

        if self.len_vector(&point_zero) == 0.0 {
            point_zero
        } else {
            Vec2D::new(
                self.x / self.len_vector(&point_zero),
                self.y / self.len_vector(&point_zero),
            )
        }
    }

    pub fn dot(vector1: &Vec2D, vector2: &Vec2D) -> f32 {
        vector1.x * vector2.x + vector1.y * vector2.y
    }

    pub fn cross(vector1: &Vec2D, vector2: &Vec2D) -> f32 {
        vector1.x * vector2.y - vector1.y * vector2.x
    }

    pub fn normal(&self) -> Vec2D {
        Vec2D::new(-self.y, self.x)
    }

    pub fn mul_n(&self, n: f32) -> Vec2D {
        *self * Vec2D::new(n, n)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul for Vec2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Vec2D {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}
