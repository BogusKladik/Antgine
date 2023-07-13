use crate::physics_engine::types::vec2d::Vec2D;

use super::object_interface::ObjectInterface;

/// Trait for describing the movement of dynamic objects
pub trait MoveInterface: ObjectInterface {
    /// Function changes the potential characteristics of the object (without touching the current ones)
    fn tracer(&mut self, time: f32);

    /// Function changes the current characteristics of the object
    fn run(&mut self, time: f32);

    /// The sat (separating axis theorem) method is a method designed to detect collisions of convex polygons.
    /// 
    /// Returns: `None` - if objects do not collide or
    /// 
    /// `Some(Minimum Translation Vector)` - if objects collide.
    /// 
    /// p.s. `MTV(Minimum Translation Vector)` - The minimum translation vector is the shortest distance that the colliding object can be moved in order to no longer be colliding with the collidee. Most of the time, this will be the normal of the face which first impacted
    fn sat(&self, object: &dyn ObjectInterface) -> Option<(f32, Vec2D, Vec2D)>;
}
