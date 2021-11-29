use std::{thread, time};

enum Program {
    Start,
    Working,
    Exit,
}

trait ObjectInterface {
    fn setPosition(&mut self, position: Point);
    fn getPosition(&self) -> Point;
    fn getSize(&self) -> Size;
}

trait MoveInterface {
    fn run(&mut self, time: f32);
}

struct Map {
    width: u16,
    height: u16,
    object: Vec<Box<dyn ObjectInterface>>,
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Size {
    width: u16,
    height: u16
}

struct Direction {
    horizontal: f32,
    vertical: f32,
}

struct Rectangle {
    position: Point,
    size: Size,
    direction: Direction,
    speed: u16,
}

impl Map {
    fn new(width: u16, height: u16) -> Map {
        Map {
            width,
            height,
            object: Vec::<Box<dyn ObjectInterface>>::new(),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(1920, 1080)
    }
}

impl Rectangle {
    fn new(
        x: i32,
        y: i32,
        width: u16,
        height: u16,
        horizontal: f32,
        vertical: f32,
        speed: u16,
    ) -> Rectangle {
        Rectangle {
            position: Point { x, y },
            size: Size { 
                width,
                height
            },
            direction: Direction {
                horizontal,
                vertical
            },
            speed,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(0, 0, 20, 20, 1.0, 0.0, 4)
    }
}

impl ObjectInterface for Rectangle {
    fn setPosition(&mut self, position: Point) {
        self.position = position
    }

    fn getPosition(&self) -> Point {
        self.position
    }

    fn getSize(&self) -> Size {
        self.size
    }
}

fn main() {
    let program = Program::Start;
    let mut map = Map::default();
    println!("Hello, world!");
}
