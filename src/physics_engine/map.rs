use super::traits::{ObjectInterface, MoveInterface};

use super::vec2d::Vec2D;

pub struct Map {
    pub plt: Vec2D,
    pub prb: Vec2D,
    objects: Vec<Box<dyn ObjectInterface>>,
    pub dyn_objects: Vec<Box<dyn MoveInterface>>
}

impl Map {
    pub fn new(plt: Vec2D, prb: Vec2D) -> Map {
        Map {
            plt,
            prb,
            objects: Vec::<Box<dyn ObjectInterface>>::new(),
            dyn_objects: Vec::<Box<dyn MoveInterface>>::new(),
        }
    }

    pub fn run(&mut self, time: f32){
        let (a, b) = (self.plt, self.prb);
        for dyn_object in self.dyn_objects.iter_mut(){
            (**dyn_object).run(&Map::new(a, b), time);
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(Vec2D::new(0.0, 0.0), Vec2D::new(1920.0, 1080.0))
    }
}