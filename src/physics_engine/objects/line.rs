// TODO: Add functions for lines to make a map constraint out of lines
use std::{collections::HashMap, convert::TryInto};

use crate::physics_engine::{
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::{angle::Angle, vec2d::Vec2D},
};

pub struct Line {
    position: HashMap<String, Vec2D>,
    vertex: HashMap<String, [Vec2D; 2]>,
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

impl Line {
    pub fn new(
        first_point: Vec2D,
        second_point: Vec2D,
        mass: f32,
        elasticity: f32,
        velocity: Vec2D,
        friction: f32,
        angle_velocity: f32,
        angle_friction: f32,
    ) -> Line {
        let vertex = HashMap::from([
            ("current".to_string(), [first_point, second_point]),
            ("potential".to_string(), [first_point, second_point]),
        ]);
        let direction = HashMap::from([
            ("current".to_string(), (second_point - first_point).unit()),
            ("sample".to_string(), (second_point - first_point).unit()),
        ]);
        let position = HashMap::from([
            (
                "current".to_string(),
                Vec2D::new(
                    (first_point.x + second_point.x) / 2.0,
                    (first_point.y + second_point.y) / 2.0,
                ),
            ),
            (
                "potential".to_string(),
                Vec2D::new(
                    (first_point.x + second_point.x) / 2.0,
                    (first_point.y + second_point.y) / 2.0,
                ),
            ),
        ]);
        let size = Vec2D::new(first_point.len_vector(&second_point), 0.0);
        let inertia = mass * size.x.powf(2.0) / 12.0;
        let angle = Angle::default();

        Line {
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

impl Default for Line {
    fn default() -> Self {
        Line::new(
            Vec2D::default(),
            Vec2D::new(1920.0, 0.0),
            0.0,
            0.0,
            Vec2D::default(),
            1.0,
            0.0,
            1.0,
        )
    }
}

impl ObjectInterface for Line{
    fn set_current_position(&mut self, position: Vec2D) {
        self.position.insert("current".to_string(), position);
    }

    fn get_current_position(&self) -> Vec2D {
        self.position["current"]
    }

    fn set_potential_position(&mut self, position: Vec2D) {
        self.position.insert("potential".to_string(), position);
    }

    fn get_potential_position(&self) -> Vec2D {
        self.position["potential"]
    }

    fn set_potential_vertex(&mut self, vertex: Vec<Vec2D>) {
        self.vertex.insert("potential".to_string(), vertex.try_into().unwrap());
    }

    fn get_potential_vertex(&self) -> Vec<Vec2D> {
        self.vertex["potential"].to_vec()
    }

    fn set_size(&mut self, size: Vec2D) {
        self.size = size;
        self.inertia = self.mass * self.size.x.powf(2.0) / 12.0;
    }

    fn get_size(&self) -> Vec2D {
        self.size
    }

    fn set_direction(&mut self, direction: Vec2D) {
        self.direction.insert("current".to_string(), direction);
    }

    fn get_direction(&self) -> Vec2D {
        self.direction["current"]
    }

    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
        self.inertia = self.mass * self.size.x.powf(2.0) / 12.0;
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
        self.angle = angle;
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
}
