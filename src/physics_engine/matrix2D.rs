struct Matrix2D {
    data: [[f32; 2]; 2]
}

impl Matrix2D {
    pub fn new(a11: f32, a12: f32, a21: f32, a22: f32,) -> Matrix2D {
        let data = [[a11, a12], [a21, a22]];
        Matrix2D { 
            data
        }
    }
}