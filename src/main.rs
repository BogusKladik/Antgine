extern crate moving_squares;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use std::mem;

enum Program {
    Start,
    _Working,
    _Exit,
}

use moving_squares::physics_engine;

fn main() {
    let _program = Program::Start;
    let mut map = physics_engine::map::Map::default();
    let a = physics_engine::rectangle::Rectangle::default();
    println!("{:?} {:?}", physics_engine::traits::ObjectInterface::get_position(&a), physics_engine::traits::ObjectInterface::get_size(&a));
    println!("{}", mem::size_of_val(&a));
    map.dyn_objects.push(Box::new(a));
    loop {
        let now = Instant::now();
        sleep(Duration::from_millis(10));
        map.run(Instant::now().duration_since(now).as_secs_f32());
    }
}
