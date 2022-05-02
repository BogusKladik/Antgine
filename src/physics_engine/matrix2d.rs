use super::vec2d::Vec2D;

use super::angle::Angle;

pub struct Matrix2D {
    data: [[f32; 2]; 2],
}

impl Default for Matrix2D {
    fn default() -> Self {
        Matrix2D::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Matrix2D {
    pub fn get_matrix(&self) -> Vec<Vec<f32>> {
        vec![self.data[0].to_vec(), self.data[1].to_vec()]
    }

    pub fn set_matrix(&mut self, matrix: Vec<Vec<f32>>) {
        if matrix.len() != 2 || matrix[0].len() != 2 || matrix[1].len() != 2 {
            panic!("Incorrect size matrix(right size is 2)")
        }

        self.data[0][0] = matrix[0][0];
        self.data[0][1] = matrix[0][1];
        self.data[1][0] = matrix[1][0];
        self.data[1][1] = matrix[1][1];
    }

    pub fn rotation_matrix(angle: &mut Angle) -> Self {
        Matrix2D::new(
            angle.get_radian().cos(),
            -angle.get_radian().sin(),
            angle.get_radian().sin(),
            angle.get_radian().cos(),
        )
    }

    pub fn new(a11: f32, a12: f32, a21: f32, a22: f32) -> Self {
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
