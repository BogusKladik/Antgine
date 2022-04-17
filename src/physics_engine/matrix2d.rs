use super::vec2d::Vec2D;

struct Matrix2D {
    data: [[f32; 2]; 2],
}

impl Default for Matrix2D {
    fn default() -> Self {
        Matrix2D::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Matrix2D {
    pub fn new(a11: f32, a12: f32, a21: f32, a22: f32) -> Matrix2D {
        let data = [[a11, a12], [a21, a22]];
        Matrix2D { data }
    }

    pub fn multiply_vec2d(&self, vector: &Vec2D) -> Vec2D {
        Vec2D::new(
            self.data[0][0] * vector.x + self.data[0][1] * vector.y,
            self.data[1][0] * vector.x + self.data[1][1] * vector.y,
        )
    }
}
