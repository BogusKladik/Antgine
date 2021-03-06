use super::{
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::vec2d::Vec2D,
};

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
            for j in (i + 1)..self.dyn_objects.len() {
                println!("len = {}, number1 = {}, number2 = {}", lenght_object, i, j);
                println!(
                    "{:?}",
                    (*self.dyn_objects[i]).sat(self.dyn_objects[j].as_object())
                );
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(Vec2D::new(0.0, 0.0), Vec2D::new(1920.0, 1080.0))
    }
}
