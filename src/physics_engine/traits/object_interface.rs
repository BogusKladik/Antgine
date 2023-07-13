use crate::physics_engine::types::{angle::Angle, vec2d::Vec2D};

use super::as_object::AsObject;

pub trait ObjectInterface: AsObject {
    fn set_current_position(&mut self, position: Vec2D);
    fn get_current_position(&self) -> Vec2D;
    fn set_potential_position(&mut self, position: Vec2D);
    fn get_potential_position(&self) -> Vec2D;
    fn set_potential_vertex(&mut self, vertex: Vec<Vec2D>);
    fn get_potential_vertex(&self) -> Vec<Vec2D>;
    fn set_size(&mut self, size: Vec2D);
    fn get_size(&self) -> Vec2D;
    fn set_direction(&mut self, direction: Vec2D);
    fn get_direction(&self) -> Vec2D;
    fn set_mass(&mut self, mass: f32);
    fn get_mass(&self) -> f32;
    fn get_inversion_mass(&self) -> f32;
    fn get_inertia(&self) -> f32;
    fn get_inversion_inertia(&self) -> f32;
    fn set_elasticity(&mut self, elasticity: f32);
    fn get_elasticity(&self) -> f32;
    fn set_velocity(&mut self, velocity: Vec2D);
    fn get_velocity(&self) -> Vec2D;
    fn set_friction(&mut self, friction: f32);
    fn get_friction(&self) -> f32;
    fn set_angle(&mut self, angle: Angle);
    fn get_angle(&self) -> Angle;
    fn set_angle_velocity(&mut self, angle_velocity: f32);
    fn get_angle_velocity(&self) -> f32;
    fn set_angle_friction(&mut self, angle_friction: f32);
    fn get_angle_friction(&self) -> f32;
    fn get_circumradius(&self) -> f32;
    fn get_axis(&self) -> Vec<Vec2D>;
    fn projection_on_axis(&self, axis: &Vec2D) -> (f32, f32, Vec2D);
    fn intersection_circumscribed_circles(&self, object: &dyn ObjectInterface) -> bool;
}

impl<T: ObjectInterface> AsObject for T {
    fn as_object(&self) -> &dyn ObjectInterface {
        self
    }

    fn as_mut_object(&mut self) -> &mut dyn ObjectInterface {
        self
    }
}
