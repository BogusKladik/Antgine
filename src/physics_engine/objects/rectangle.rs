use std::{collections::HashMap, convert::TryInto};

use super::super::{
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::{angle::Angle, vec2d::Vec2D, matrix2d::Matrix2D},
};

/// Rectangle structure
pub struct Rectangle {
    position: HashMap<String, Vec2D>,
    vertex: HashMap<String, [Vec2D; 4]>,
    size: Vec2D,
    direction: HashMap<String, Vec2D>,
    mass: f32,
    inertia: f32,
    elasticity: f32,
    velocity: Vec2D,
    friction: f32,
    angle: Angle,
    angle_velocity: f32,
    angle_friction: f32,
}

impl Rectangle {
    /// Creating a rectangle
    pub fn new(
        first_point: Vec2D,
        second_point: Vec2D,
        width: f32,
        mass: f32,
        elasticity: f32,
        velocity: Vec2D,
        friction: f32,
        angle_velocity: f32,
        angle_friction: f32,
    ) -> Rectangle {
        let edge = second_point - first_point;
        let size = Vec2D::new(width, edge.len_vector(&Vec2D::new(0.0, 0.0)));
        let direction = HashMap::from([
            ("current".to_string(), edge.unit()),
            ("sample".to_string(), edge.unit()),
        ]);
        let third_point = second_point + direction["current"].normal().mul_n(-size.x);
        let fourth_point = third_point + direction["current"].mul_n(-size.y);
        let vertex = HashMap::from([
            (
                "current".to_string(),
                [first_point, second_point, third_point, fourth_point],
            ),
            (
                "potential".to_string(),
                [first_point, second_point, third_point, fourth_point],
            ),
        ]);
        let position = first_point
            + direction["current"].mul_n(size.y / 2.0)
            + direction["current"].normal().mul_n(-size.x / 2.0);
        let position = HashMap::from([
            ("current".to_string(), position),
            ("potential".to_string(), position),
        ]);
        let inertia = mass * (size.x.powf(2.0) + size.y.powf(2.0)) / 12.0;
        let angle = Angle::default();

        Rectangle {
            position,
            vertex,
            size,
            direction,
            mass,
            inertia,
            elasticity,
            velocity,
            friction,
            angle,
            angle_velocity,
            angle_friction,
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
            1.0,
            Vec2D::new(100.0, 0.0),
            0.01,
            0.0,
            0.01,
        )
    }
}

impl ObjectInterface for Rectangle {
    fn set_current_position(&mut self, position: Vec2D) {
        self.position.insert("current".to_string(), position);
    }

    fn set_potential_position(&mut self, position: Vec2D) {
        self.position.insert("potential".to_string(), position);
    }

    fn get_current_position(&self) -> Vec2D {
        self.position["current"]
    }

    fn get_potential_position(&self) -> Vec2D {
        self.position["potential"]
    }

    fn get_size(&self) -> Vec2D {
        self.size
    }

    fn get_potential_vertex(&self) -> Vec<Vec2D> {
        (self.vertex["potential"]).to_vec()
    }

    fn get_direction(&self) -> Vec2D {
        self.direction["current"]
    }

    fn set_potential_vertex(&mut self, vertex: Vec<Vec2D>) {
        self.vertex
            .insert("potential".to_string(), vertex.try_into().unwrap());
    }

    fn set_size(&mut self, size: Vec2D) {
        self.size = size;
        self.inertia = self.mass * (self.size.x.powf(2.0) + self.size.y.powf(2.0)) / 12.0;
    }

    fn set_direction(&mut self, direction: Vec2D) {
        self.direction.insert("current".to_string(), direction);
    }

    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
        self.inertia = self.mass * (self.size.x.powf(2.0) + self.size.y.powf(2.0)) / 12.0;
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_inversion_mass(&self) -> f32 {
        if self.mass == 0.0 {
            0.0
        } else {
            1.0 / self.mass
        }
    }

    fn get_inertia(&self) -> f32 {
        self.inertia
    }

    fn get_inversion_inertia(&self) -> f32 {
        if self.inertia == 0.0 {
            0.0
        } else {
            1.0 / self.inertia
        }
    }

    fn set_elasticity(&mut self, elasticity: f32) {
        self.elasticity = elasticity;
    }

    fn get_elasticity(&self) -> f32 {
        self.elasticity
    }

    fn set_velocity(&mut self, velocity: Vec2D) {
        self.velocity = velocity;
    }

    fn get_velocity(&self) -> Vec2D {
        self.velocity
    }

    fn set_friction(&mut self, friction: f32) {
        self.friction = friction;
    }

    fn get_friction(&self) -> f32 {
        self.friction
    }

    fn set_angle(&mut self, angle: Angle) {
        self.angle = angle
    }

    fn get_angle(&self) -> Angle {
        self.angle
    }

    fn set_angle_velocity(&mut self, angle_velocity: f32) {
        self.angle_velocity = angle_velocity;
    }

    fn get_angle_velocity(&self) -> f32 {
        self.angle_velocity
    }

    fn set_angle_friction(&mut self, angle_friction: f32) {
        self.angle_friction = angle_friction;
    }

    fn get_angle_friction(&self) -> f32 {
        self.angle_friction
    }

    fn get_circumradius(&self) -> f32 {
        (self.size.x.powf(2.0) + self.size.y.powf(2.0)).powf(0.5) / 2.0
    }

    fn get_axis(&self) -> Vec<Vec2D> {
        vec![self.get_direction().normal(), self.get_direction()]
    }
}

impl MoveInterface for Rectangle {
    fn tracer(&mut self, time: f32) {
        self.position.insert(
            "potential".to_string(),
            self.position["potential"] + Vec2D::new(self.velocity.x * time, self.velocity.y * time),
        );

        self.angle = Angle::new(self.angle.get_radian() + self.angle_velocity);
        self.angle_velocity *= 1.0 - self.angle_friction;
        let rotation_matrix = Matrix2D::rotation_matrix(&self.angle);
        self.direction.insert(
            "current".to_string(),
            rotation_matrix.multiply_vec2d(&self.direction["sample"])
        );

        self.vertex.insert(
            "potential".to_string(),
            [
                self.position["potential"]
                    + self.direction["current"].mul_n(-self.size.y / 2.0)
                    + self.direction["current"].normal().mul_n(-self.size.x / 2.0),
                self.position["potential"]
                    + self.direction["current"].mul_n(-self.size.y / 2.0)
                    + self.direction["current"].normal().mul_n(self.size.x / 2.0),
                self.position["potential"]
                    + self.direction["current"].mul_n(self.size.y / 2.0)
                    + self.direction["current"].normal().mul_n(-self.size.x / 2.0),
                self.position["potential"]
                    + self.direction["current"].mul_n(self.size.y / 2.0)
                    + self.direction["current"].normal().mul_n(self.size.x / 2.0),
            ],
        );
    }

    fn run(&mut self, time: f32) {
        self.tracer(time);

        self.position
            .insert("current".to_string(), self.position["potential"]);
        self.vertex.insert(
            "current".to_string(),
            [
                self.vertex["potential"][0],
                self.vertex["potential"][1],
                self.vertex["potential"][2],
                self.vertex["potential"][3],
            ],
        );

        // Debug
        println!(
            "x = {}, y = {}, velocity = {:?}, direction = {:?}, size = {:?}, time = {}",
            self.position["current"].x,
            self.position["current"].y,
            self.velocity,
            self.direction,
            self.size,
            time
        );
    }

    fn sat(&self, object: &dyn ObjectInterface) -> Option<(f32, Vec2D, Vec2D)> {
        /// Auxiliary function for finding the projections of points on the axis
        fn projection_on_axis(axis: &Vec2D, object: &dyn ObjectInterface) -> (f32, f32, Vec2D) {
            let vertices = object.get_potential_vertex();

            // initializes the min and max location of the point relative to the axis, and the vertex where the collision is
            let mut min = Vec2D::dot(axis, &object.get_potential_vertex()[0]);
            let mut max = min;
            let mut collision_vertex = vertices[0];

            // considers all vertices to find the answer
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

        // creates an array of axes of 2 objects
        let mut axes = self.get_axis();
        axes.extend(object.get_axis().iter());

        // initializes variables responsible for the minimum filling of one object with another, the smallest axis where the collision occurs, and whether the object in question is the main one
        let mut min_overlap = None;
        let mut smallest_axis = Vec2D::default();
        let mut main_object = false;

        // looks at the projections if there is an overflow
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

            // searches for the minimum overflow among overflows
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

        // changes contact vertex and minor axis, depending on whether the object is the main one
        let contact_vertex;
        if main_object {
            contact_vertex = projection_on_axis(&smallest_axis, self).2;
        } else {
            contact_vertex = projection_on_axis(&smallest_axis, object).2;
            smallest_axis = smallest_axis.mul_n(-1.0);
        }

        Some((min_overlap.unwrap(), smallest_axis, contact_vertex))
    }

    fn intersection_circumscribed_circles(&self, object: &dyn ObjectInterface) -> bool {
        self.get_potential_position()
            .len_vector(&object.get_potential_position())
            < self.get_circumradius() + object.get_circumradius()
    }
}
