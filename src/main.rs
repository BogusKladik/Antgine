extern crate antgine;

use std::{
    mem,
    thread::sleep,
    time::{Duration, Instant},
};

use antgine::physics_engine::{self, types::vec2d::Vec2D};

enum Program {
    Start,
    _Working,
    _Exit,
}

fn main() {
    let _program = Program::Start;
    let mut map = physics_engine::map::Map::default();
    let a = physics_engine::objects::rectangle::Rectangle::default();
    let b = physics_engine::objects::rectangle::Rectangle::new(
        Vec2D::new(1900.0, 0.0),
        Vec2D::new(1900.0, 20.0),
        20.0,
        10.0,
        Vec2D::new(-100.0, 0.0),
    );
    println!(
        "{:?} {:?}",
        physics_engine::traits::ObjectInterface::ObjectInterface::get_current_position(&a),
        physics_engine::traits::ObjectInterface::ObjectInterface::get_size(&a)
    );
    println!("{}", mem::size_of_val(&a));
    map.dyn_objects.push(Box::new(a));
    map.dyn_objects.push(Box::new(b));
    loop {
        let now = Instant::now();
        sleep(Duration::from_millis(10));
        map.run(Instant::now().duration_since(now).as_secs_f32());
    }
}
