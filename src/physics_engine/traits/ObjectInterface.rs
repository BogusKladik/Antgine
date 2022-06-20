use crate::physics_engine::types::vec2d::Vec2D;

use super::AsObject::AsObject;

pub trait ObjectInterface: AsObject {
    fn set_current_position(&mut self, position: Vec2D);
    fn set_potential_position(&mut self, position: Vec2D);
    fn get_current_position(&self) -> Vec2D;
    fn get_potential_position(&self) -> Vec2D;
    fn get_size(&self) -> Vec2D;
    fn get_potential_vertex(&self) -> Vec<Vec2D>;
    fn get_direction(&self) -> Vec2D;
}

impl<T: ObjectInterface> AsObject for T {
    fn as_object(&self) -> &dyn ObjectInterface {
        self
    }
}
