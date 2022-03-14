use super::traits::{ObjectInterface, MoveInterface};

use super::vec2d::Vec2D;

use super::map::Map;


pub struct Rectangle {
    position: Vec2D,
    size: Vec2D,
    direction: Vec2D,
    speed: f32,
}

impl Rectangle {
    pub fn new(
        position: Vec2D,
        size: Vec2D,
        direction: Vec2D,
        speed: f32,
    ) -> Rectangle {
        Rectangle {
            position,
            size,
            direction,
            speed,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(Vec2D::new(0.0, 0.0), Vec2D::new(20.0, 20.0), Vec2D::new(0.33, -0.66), 600.4)
    }
}

impl ObjectInterface for Rectangle {
    fn set_position(&mut self, position: Vec2D) {
        self.position = position
    }

    fn get_position(&self) -> Vec2D {
        self.position
    }

    fn get_size(&self) -> Vec2D {
        self.size
    }
}

impl MoveInterface for Rectangle {
    fn run(&mut self, map: &Map, time: f32) {
        let mut point = self.position
            + Vec2D {
                x: self.direction.x * self.speed * time,
                y: self.direction.y * self.speed * time,
            };

        loop {
            match point {
                Vec2D { x, .. } if map.plt.x > x || x + self.size.x >= map.prb.x => {
                    self.direction.x = -self.direction.x;

                    if map.plt.x > x {
                        point.x = 2.0 * map.plt.x - x;
                    } else if x + self.size.x > map.prb.x {
                        point.x = 2.0 * map.prb.x - x - 2.0 * self.size.x;
                    } else {
                        self.set_position(point);
                        break;
                    }
                }
                Vec2D { y, .. } if map.plt.y > y || y + self.size.y >= map.prb.y => {
                    self.direction.y = -self.direction.y;

                    if map.plt.y > y {
                        point.y = 2.0 * map.plt.y - y;
                    } else if y + self.size.y >= map.prb.y {
                        point.y = 2.0 * map.prb.y - y - 2.0 * self.size.y;
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
        println!("x = {}, y = {}", self.position.x, self.position.y);
    }

    fn check_collision(&self, plt: Vec2D, prb: Vec2D) -> bool {
        let collision_x =
            self.position.x + self.size.x >= plt.x && prb.x >= self.position.x;

        let collision_y =
            self.position.y + self.size.y >= plt.y && prb.y >= self.position.y;

        collision_x && collision_y
    }
}