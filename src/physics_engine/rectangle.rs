use super::traits::{MoveInterface, ObjectInterface};

use super::vec2d::Vec2D;

use super::map::Map;

pub struct Rectangle {
    position: [Vec2D; 2],
    size: Vec2D,
    mass: f32,
    direction: Vec2D,
    speed: f32,
}

impl Rectangle {
    pub fn new(
        position: [Vec2D; 2],
        size: Vec2D,
        mass: f32,
        direction: Vec2D,
        speed: f32,
    ) -> Rectangle {
        Rectangle {
            position,
            size,
            mass,
            direction,
            speed,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(
            [
                Vec2D::new(1586.7632, 1058.5038),
                Vec2D::new(1586.7632, 1058.5038),
            ],
            Vec2D::new(20.0, 20.0),
            10.0,
            Vec2D::new(-0.33, 0.66),
            600.4,
        )
    }
}

impl ObjectInterface for Rectangle {
    fn set_current_position(&mut self, position: Vec2D) {
        self.position[0] = position
    }

    fn set_potential_position(&mut self, position: Vec2D) {
        self.position[1] = position
    }

    fn get_current_position(&self) -> Vec2D {
        self.position[0]
    }

    fn get_potential_position(&self) -> Vec2D {
        self.position[1]
    }

    fn get_size(&self) -> Vec2D {
        self.size
    }
}

impl MoveInterface for Rectangle {
    fn tracer(&mut self, map: &Map, time: f32) {
        self.position[1] = self.position[0]
            + Vec2D::new(
                self.direction.x * self.speed * time,
                self.direction.y * self.speed * time,
            );

        loop {
            match self.position[1] {
                Vec2D { x, .. } if map.plt.x > x || x + self.size.x >= map.prb.x => {
                    self.direction.x = -self.direction.x;

                    if map.plt.x > x {
                        self.position[0] = Vec2D::cross_pointll(
                            [self.position[0], self.position[1]],
                            [map.plt, Vec2D::new(map.plt.x, map.prb.y)])
                        .unwrap_or_else(|| Vec2D::new(map.plt.x, self.position[0].y));

                        self.position[1].x = 2.0 * map.plt.x - x;
                    } else if x + self.size.x > map.prb.x {
                        self.position[0] = Vec2D::cross_pointll(
                            [self.position[0], self.position[1]],
                            [map.prb, Vec2D::new(map.prb.x, map.plt.y)])
                        .unwrap_or_else(|| Vec2D::new(map.prb.x, self.position[0].y))
                            - Vec2D::new(self.size.x, 0.0);

                        self.position[1].x = 2.0 * map.prb.x - x - 2.0 * self.size.x;
                    } else {
                        break;
                    }
                }
                Vec2D { y, .. } if map.plt.y > y || y + self.size.y >= map.prb.y => {
                    self.direction.y = -self.direction.y;

                    if map.plt.y > y {
                        self.position[0] = Vec2D::cross_pointll(
                            [self.position[0], self.position[1]],
                            [map.plt, Vec2D::new(map.prb.x, map.plt.y)])
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, map.plt.y));

                        self.position[1].y = 2.0 * map.plt.y - y;
                    } else if y + self.size.y >= map.prb.y {
                        self.position[0] = Vec2D::cross_pointll(
                            [self.position[0], self.position[1]],
                            [map.prb, Vec2D::new(map.plt.x, map.prb.y)])
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, map.prb.y))
                            - Vec2D::new(0.0, self.size.y);

                        self.position[1].y = 2.0 * map.prb.y - y - 2.0 * self.size.y;
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn run(&mut self, map: &Map, time: f32) {
        self.tracer(map, time);
        
        self.position[0] = self.position[1];
        println!(
            "x = {}, y = {}, speed = {}, direction = {:?}, size = {:?}, time = {}",
            self.position[1].x, self.position[1].y, self.speed, self.direction, self.size, time
        );
    }

    fn check_collision(&self, plt: Vec2D, prb: Vec2D) -> bool {
        let collision_x = self.position[1].x + self.size.x >= plt.x && prb.x >= self.position[1].x;

        let collision_y = self.position[1].y + self.size.y >= plt.y && prb.y >= self.position[1].y;

        collision_x && collision_y
    }
}
