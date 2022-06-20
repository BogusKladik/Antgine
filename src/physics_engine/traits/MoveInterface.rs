use crate::physics_engine::types::vec2d::Vec2D;

use super::ObjectInterface::ObjectInterface;

pub trait MoveInterface: ObjectInterface {
    fn tracer(&mut self, time: f32);
    fn run_with_boundaries(&mut self, plt: &Vec2D, prb: &Vec2D);
    fn run(&mut self, plt: Vec2D, prb: Vec2D, time: f32);
    fn sat(&self, object: &dyn ObjectInterface) -> Option<(f32, Vec2D, Vec2D)>;
}
