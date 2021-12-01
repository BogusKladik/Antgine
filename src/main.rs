use std::{
    thread::sleep,
    time::{Duration, Instant},
};

enum Program {
    Start,
    Working,
    Exit,
}

trait ObjectInterface {
    fn set_position(&mut self, position: Point);
    fn get_position(&self) -> Point;
    fn get_size(&self) -> Size;
}

trait MoveInterface {
    fn run(&mut self, map: &Map, time: f32);
}

struct Map {
    plt: Point,
    prb: Point,
    objects: Vec<Box<dyn ObjectInterface>>,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Size {
    width: u16,
    height: u16,
}

struct Direction {
    horizontal: f32,
    vertical: f32,
}

struct Rectangle {
    position: Point,
    size: Size,
    direction: Direction,
    speed: f32,
}

impl Map {
    fn new(plt: Point, prb: Point) -> Map {
        Map {
            plt,
            prb,
            objects: Vec::<Box<dyn ObjectInterface>>::new(),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new(Point::new(0, 0), Point::new(1920, 1080))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
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
        speed: f32,
    ) -> Rectangle {
        Rectangle {
            position: Point { x, y },
            size: Size { width, height },
            direction: Direction {
                horizontal,
                vertical,
            },
            speed,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(0, 0, 20, 20, 0.33, -0.66, 600.4)
    }
}

impl ObjectInterface for Rectangle {
    fn set_position(&mut self, position: Point) {
        self.position = position
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

impl MoveInterface for Rectangle {
    fn run(&mut self, map: &Map, time: f32) {
        let mut point = self.position;
        point.add(
            (self.direction.horizontal * self.speed * time) as i32,
            (self.direction.vertical * self.speed * time) as i32,
        );
        loop {
            match point {
                Point { x, .. } if map.plt.x > x || x + (self.size.width as i32) >= map.prb.x => {
                    self.direction.horizontal = -self.direction.horizontal;

                    if map.plt.x > x {
                        point.x = 2 * map.plt.x - x;
                    } else if x + (self.size.width as i32) > map.prb.x {
                        point.x = 2 * map.prb.x - x - 2 * (self.size.width as i32);
                    } else {
                        self.set_position(point);
                        break;
                    }
                }
                Point { y, .. } if map.plt.y > y || y + (self.size.height as i32) >= map.prb.y => {
                    self.direction.vertical = -self.direction.vertical;

                    if map.plt.y > y {
                        point.y = 2 * map.plt.y - y;
                    } else if y + (self.size.height as i32) >= map.prb.y {
                        point.y = 2 * map.prb.y - y - 2 * (self.size.height as i32);
                    } else {
                        self.set_position(point);
                        break;
                    }
                }
                _ => {
                    self.set_position(point);
                    break;
                }
            }
        }
    }
}

fn main() {
    let program = Program::Start;
    let mut map = Map::default();
    let mut a = Rectangle::default();
    println!("{:?} {:?}", a.get_position(), a.get_size());
    loop {
        let now = Instant::now();
        sleep(Duration::from_millis(100));
        a.run(&map, Instant::now().duration_since(now).as_secs_f32());
        println!("{:?} {:?} t = {:?}", a.get_position(), a.get_size(), now);
    }
}
