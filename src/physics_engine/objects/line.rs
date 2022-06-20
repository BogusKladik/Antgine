// TODO: Add functions for lines to make a map constraint out of lines
use std::collections::HashMap;

use crate::physics_engine::{
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::{vec2d::Vec2D, angle::Angle},
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
