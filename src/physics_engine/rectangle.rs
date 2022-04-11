use super::traits::{MoveInterface, ObjectInterface};

use super::vec2d::Vec2D;

pub struct Rectangle {
    position: [Vec2D; 2],
    vertex: [[Vec2D; 4]; 2],
    size: Vec2D,
    mass: f32,
    direction: [Vec2D; 2],
    velocity: Vec2D,
}

impl Rectangle {
    pub fn new(
        first_point: Vec2D,
        second_point: Vec2D,
        width: f32,
        mass: f32,
        velocity: Vec2D,
    ) -> Rectangle {
        let edge = second_point - first_point;
        let size = Vec2D::new(width, edge.len_vector(&Vec2D::new(0.0, 0.0)));
        let direction = [edge.unit(), edge.unit()];
        let third_point = second_point + direction[0].normal().mul_n(-size.x);
        let fourth_point = third_point + direction[0].mul_n(-size.y);
        let vertex = [[first_point, second_point, third_point, fourth_point],
        [first_point, second_point, third_point, fourth_point]];
        let position = first_point + direction[0].mul_n(size.y / 2.0) + direction[0].normal().mul_n(-size.x / 2.0);
        let position = [position, position];

        Rectangle {
            position,
            vertex,
            size,
            mass,
            direction,
            velocity,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(
            Vec2D::new(0.0, 0.0),
            Vec2D::new(0.0, 20.0),
            20.0,
            10.0,
            Vec2D::new(100.0, 0.0),
        )
    }
}

impl ObjectInterface for Rectangle {
    fn set_current_position(&mut self, position: Vec2D) {
        self.position[0] = position
    }

    fn set_potential_position(&mut self, position: Vec2D) {
        self.position[1] = position
    }

    fn get_current_position(&self) -> Vec2D {
        self.position[0]
    }

    fn get_potential_position(&self) -> Vec2D {
        self.position[1]
    }

    fn get_size(&self) -> Vec2D {
        self.size
    }

    fn get_potential_vertex(&self) -> &[Vec2D] {
        &self.vertex[1]
    }

    fn get_direction(&self) -> Vec2D {
        self.direction[0]
    }
}

impl MoveInterface for Rectangle {
    fn tracer(&mut self, time: f32) {
        self.position[1] += Vec2D::new(
            self.velocity.x * time,
            self.velocity.y * time,
        );
    }

    fn run_with_boundaries(&mut self, plt: &Vec2D, prb: &Vec2D) {
        loop {
            match self.position[1] {
                Vec2D { x, .. }
                    if plt.x > x - self.size.x / 2.0 || x + self.size.x / 2.0 >= prb.x =>
                {
                    self.velocity.x = -self.velocity.x;

                    if plt.x > x - self.size.x / 2.0 {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [plt, &Vec2D::new(plt.x, prb.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(plt.x, self.position[0].y))
                            + Vec2D::new(self.size.x / 2.0, 0.0);

                        self.position[1].x = 2.0 * plt.x - (x - self.size.x);
                    } else {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [prb, &Vec2D::new(prb.x, plt.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(prb.x, self.position[0].y))
                            - Vec2D::new(self.size.x / 2.0, 0.0);

                        self.position[1].x = 2.0 * prb.x - (x + self.size.x);
                    }
                }
                Vec2D { y, .. }
                    if plt.y > y - self.size.y / 2.0 || y + self.size.y / 2.0 >= prb.y =>
                {
                    self.velocity.y = -self.velocity.y;

                    if plt.y > y - self.size.y / 2.0 {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [plt, &Vec2D::new(prb.x, plt.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, plt.y))
                            + Vec2D::new(0.0, self.size.y / 2.0);

                        self.position[1].y = 2.0 * plt.y - (y - self.size.y);
                    } else {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [prb, &Vec2D::new(plt.x, prb.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, prb.y))
                            - Vec2D::new(0.0, self.size.y / 2.0);

                        self.position[1].y = 2.0 * prb.y - (y + self.size.y);
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn run(&mut self, plt: Vec2D, prb: Vec2D, time: f32) {
        // let time = 0.0101444;
        // let time = 0.25;
        self.tracer(time);
        self.run_with_boundaries(&plt, &prb);

        self.vertex[1][0] = self.position[1] + self.direction[0].mul_n(-self.size.y / 2.0) + self.direction[0].normal().mul_n(-self.size.x / 2.0);
        self.vertex[1][1] = self.position[1] + self.direction[0].mul_n(-self.size.y / 2.0) + self.direction[0].normal().mul_n(self.size.x / 2.0);
        self.vertex[1][2] = self.position[1] + self.direction[0].mul_n(self.size.y / 2.0) + self.direction[0].normal().mul_n(-self.size.x / 2.0);
        self.vertex[1][3] = self.position[1] + self.direction[0].mul_n(self.size.y / 2.0) + self.direction[0].normal().mul_n(self.size.x / 2.0);
        
        self.position[0] = self.position[1];
        self.vertex[0][0] = self.vertex[1][0];
        self.vertex[0][1] = self.vertex[1][1];
        self.vertex[0][2] = self.vertex[1][2];
        self.vertex[0][3] = self.vertex[1][3];
        println!(
            "x = {}, y = {}, velocity = {:?}, direction = {:?}, size = {:?}, time = {}",
            self.position[1].x, self.position[1].y, self.velocity, self.direction, self.size, time
        );
    }

    fn check_collision(&self, position: Vec2D, size: Vec2D) -> bool {
        if (self.position[1].x - position.x).abs() > (self.size.x + size.x) / 2.0 {
            return false;
        }

        if (self.position[1].y - position.y).abs() > (self.size.y + size.y) / 2.0 {
            return false;
        }

        true
    }

    fn sat(&self, object: &dyn MoveInterface) -> bool {
        fn projection_on_axis(axis: &Vec2D, object: &dyn MoveInterface) -> (f32, f32) {
            let mut min = Vec2D::dot(axis, &object.get_potential_vertex()[0]);
            let mut max = min;
    
            for view_vertex in object.get_potential_vertex() {
                let p = Vec2D::dot(axis, view_vertex);
    
                if p < min {
                    min = p
                }
    
                if p > max {
                    max = p
                }
            }
            (max, min)
        }

        let axis1 = [self.get_direction().normal(), self.direction[0]];
        let axis2 = [object.get_direction().normal(), object.get_direction()];

        for axis in axis1 {
            let (max1, min1) = projection_on_axis(&axis, self);
            let (max2, min2) = projection_on_axis(&axis, object);

            let overlap = max1.min(max2) - min1.max(min2);
            if overlap <= 0.0 {
                return false
            }
        }

        for axis in axis2 {
            let (max1, min1) = projection_on_axis(&axis, self);
            let (max2, min2) = projection_on_axis(&axis, object);

            let overlap = max1.min(max2) - min1.max(min2);
            if overlap <= 0.0 {
                return false
            }
        }

        true
    }
}
