use super::vec2d::Vec2D;

pub trait ObjectInterface {
    fn set_current_position(&mut self, position: Vec2D);
    fn set_potential_position(&mut self, position: Vec2D);
    fn get_current_position(&self) -> Vec2D;
    fn get_potential_position(&self) -> Vec2D;
    fn get_size(&self) -> Vec2D;
    fn get_potential_vertex(&self) -> &[Vec2D];
    fn get_direction(&self) -> Vec2D;
}

pub trait MoveInterface: ObjectInterface {
    fn tracer(&mut self, time: f32);
    fn run_with_boundaries(&mut self, plt: &Vec2D, prb: &Vec2D);
    fn run(&mut self, plt: Vec2D, prb: Vec2D, time: f32);
    fn check_collision(&self, position: Vec2D, size: Vec2D) -> bool;
    fn sat(&self, object: &dyn MoveInterface) -> bool;
}
