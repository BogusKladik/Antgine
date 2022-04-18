use super::traits::{MoveInterface, ObjectInterface};

use super::vec2d::Vec2D;

pub struct Map {
    plt: Vec2D,
    prb: Vec2D,
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

    pub fn get_boundaries(&self) -> (Vec2D, Vec2D) {
        (self.plt, self.prb)
    }

    pub fn run(&mut self, time: f32) {
        let (plt, prb) = self.get_boundaries();
        let lenght_object = self.dyn_objects.len();
        for i in 0..self.dyn_objects.len() {
            (*self.dyn_objects[i]).run(plt, prb, time);
            println!("len = {}, number = {}", lenght_object, i);
            for j in 0..self.dyn_objects.len() {
                if i != j {
                    println!("{:?}", (*self.dyn_objects[i]).sat(&(*self.dyn_objects[j])));
                }
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(Vec2D::new(0.0, 0.0), Vec2D::new(1920.0, 1080.0))
    }
}
