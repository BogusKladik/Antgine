use super::vec2d::Vec2D;

use super::map::Map;

pub trait ObjectInterface {
    fn set_current_position(&mut self, position: Vec2D);
    fn set_potential_position(&mut self, position: Vec2D);
    fn get_current_position(&self) -> Vec2D;
    fn get_potential_position(&self) -> Vec2D;
    fn get_size(&self) -> Vec2D;
}

pub trait MoveInterface: ObjectInterface {
    fn tracer(&mut self, time: f32);
    fn run_with_boundaries(&mut self, map: &Map);
    fn run(&mut self, map: &Map, time: f32);
    fn check_collision(&self, /*plt: Vec2D, prb: Vec2D*/ position: Vec2D, size: Vec2D) -> bool;
}
