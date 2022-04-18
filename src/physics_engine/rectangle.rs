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
        let vertex = [
            [first_point, second_point, third_point, fourth_point],
            [first_point, second_point, third_point, fourth_point],
        ];
        let position = first_point
            + direction[0].mul_n(size.y / 2.0)
            + direction[0].normal().mul_n(-size.x / 2.0);
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

    fn get_potential_vertex(&self) -> Vec<Vec2D> {
        (self.vertex[1]).to_vec()
    }

    fn get_direction(&self) -> Vec2D {
        self.direction[0]
    }
}

impl MoveInterface for Rectangle {
    fn tracer(&mut self, time: f32) {
        self.position[1] += Vec2D::new(self.velocity.x * time, self.velocity.y * time);
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
        self.tracer(time);
        self.run_with_boundaries(&plt, &prb);

        self.vertex[1][0] = self.position[1]
            + self.direction[0].mul_n(-self.size.y / 2.0)
            + self.direction[0].normal().mul_n(-self.size.x / 2.0);
        self.vertex[1][1] = self.position[1]
            + self.direction[0].mul_n(-self.size.y / 2.0)
            + self.direction[0].normal().mul_n(self.size.x / 2.0);
        self.vertex[1][2] = self.position[1]
            + self.direction[0].mul_n(self.size.y / 2.0)
            + self.direction[0].normal().mul_n(-self.size.x / 2.0);
        self.vertex[1][3] = self.position[1]
            + self.direction[0].mul_n(self.size.y / 2.0)
            + self.direction[0].normal().mul_n(self.size.x / 2.0);

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

    fn sat(&self, object: &dyn MoveInterface) -> Option<(f32, Vec2D, Vec2D)> {
        fn projection_on_axis(axis: &Vec2D, object: &dyn MoveInterface) -> (f32, f32, Vec2D) {
            let vertices = object.get_potential_vertex();

            let mut min = Vec2D::dot(axis, &object.get_potential_vertex()[0]);
            let mut max = min;
            let mut collision_vertex = vertices[0];

            for view_vertex in vertices {
                let p = Vec2D::dot(axis, &view_vertex);

                if p < min {
                    min = p;
                    collision_vertex = view_vertex;
                }

                if p > max {
                    max = p
                }
            }
            (max, min, collision_vertex)
        }

        fn find_axes(object1: &dyn MoveInterface, object2: &dyn MoveInterface) -> Vec<Vec2D> {
            let mut axis = Vec::new();

            let mut axis1 = vec![object1.get_direction().normal(), object1.get_direction()];
            let mut axis2 = vec![object2.get_direction().normal(), object2.get_direction()];

            axis.append(&mut axis1);
            axis.append(&mut axis2);

            axis
        }

        let mut axes = find_axes(self, object);
        let mut min_overlap = None;
        let mut smallest_axis = Vec2D::default();
        let mut main_object = false;

        for i in 0..axes.len() {
            let (max1, min1, _) = projection_on_axis(&axes[i], self);
            let (max2, min2, _) = projection_on_axis(&axes[i], object);

            let mut overlap = max1.min(max2) - min1.max(min2);
            if overlap <= 0.0 {
                return None;
            }

            if (max1 > max2 && min1 < min2) || (max1 < max2 && min1 > min2) {
                let min = (min1 - min2).abs();
                let max = (max1 - max2).abs();

                if min < max {
                    overlap += min
                } else {
                    overlap += max;
                    axes[i] = axes[i].mul_n(-1.0);
                }
            }

            match min_overlap {
                Some(j) if overlap >= j => (),
                _ => {
                    min_overlap = Some(overlap);
                    smallest_axis = axes[i];

                    if i < 2 {
                        main_object = false;
                        if max1 > max2 {
                            smallest_axis = smallest_axis.mul_n(-1.0)
                        }
                    } else {
                        main_object = true;
                        if max1 < max2 {
                            smallest_axis = smallest_axis.mul_n(-1.0)
                        }
                    }
                }
            }
        }

        let contact_vertex;
        if main_object {
            contact_vertex = projection_on_axis(&smallest_axis, self).2;
        } else {
            contact_vertex = projection_on_axis(&smallest_axis, object).2;
            smallest_axis = smallest_axis.mul_n(-1.0);
        }

        Some((min_overlap.unwrap(), smallest_axis, contact_vertex))
    }
}
