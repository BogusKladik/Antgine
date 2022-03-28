use super::traits::{MoveInterface, ObjectInterface};

use super::vec2d::Vec2D;

pub struct Map {
    pub plt: Vec2D,
    pub prb: Vec2D,
    objects: Vec<Box<dyn ObjectInterface>>,
    pub dyn_objects: Vec<Box<dyn MoveInterface>>,
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

    pub fn run(&mut self, time: f32) {
        let (a, b) = (self.plt, self.prb);
        let lenght_object = self.dyn_objects.len();
        for (i, dyn_object) in self.dyn_objects.iter_mut().enumerate() {
            (**dyn_object).run(&Map::new(a, b), time);
            println!("len = {}, number = {}", lenght_object, i);
            // for (j, another_dyn_object) in self.dyn_objects.iter_mut().enumerate(){

            // }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(Vec2D::new(0.0, 0.0), Vec2D::new(1920.0, 1080.0))
    }
}
