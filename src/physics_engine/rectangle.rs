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
            [Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 0.0)],
            Vec2D::new(20.0, 20.0),
            10.0,
            Vec2D::new(1.0, 0.0),
            100.0,
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
    fn tracer(&mut self, time: f32) {
        self.position[1] += Vec2D::new(
            self.direction.x * self.speed * time,
            self.direction.y * self.speed * time,
        );
    }

    fn run_with_boundaries(&mut self, map: &Map) {
        loop {
            match self.position[1] {
                Vec2D { x, .. }
                    if map.plt.x > x - self.size.x / 2.0 || x + self.size.x / 2.0 >= map.prb.x =>
                {
                    self.direction.x = -self.direction.x;

                    if map.plt.x > x - self.size.x / 2.0 {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [&map.plt, &Vec2D::new(map.plt.x, map.prb.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(map.plt.x, self.position[0].y))
                            + Vec2D::new(self.size.x / 2.0, 0.0);

                        self.position[1].x = 2.0 * map.plt.x - (x - self.size.x);
                    } else {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [&map.prb, &Vec2D::new(map.prb.x, map.plt.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(map.prb.x, self.position[0].y))
                            - Vec2D::new(self.size.x / 2.0, 0.0);

                        self.position[1].x = 2.0 * map.prb.x - (x + self.size.x);
                    }
                }
                Vec2D { y, .. }
                    if map.plt.y > y - self.size.y / 2.0 || y + self.size.y / 2.0 >= map.prb.y =>
                {
                    self.direction.y = -self.direction.y;

                    if map.plt.y > y - self.size.y / 2.0 {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [&map.plt, &Vec2D::new(map.prb.x, map.plt.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, map.plt.y))
                            + Vec2D::new(0.0, self.size.y / 2.0);

                        self.position[1].y = 2.0 * map.plt.y - (y - self.size.y);
                    } else {
                        self.position[0] = Vec2D::cross_pointll(
                            [&self.position[0], &self.position[1]],
                            [&map.prb, &Vec2D::new(map.plt.x, map.prb.y)],
                        )
                        .unwrap_or_else(|| Vec2D::new(self.position[0].x, map.prb.y))
                            - Vec2D::new(0.0, self.size.y / 2.0);

                        self.position[1].y = 2.0 * map.prb.y - (y + self.size.y);
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn run(&mut self, map: &Map, time: f32) {
        self.tracer(time);
        self.run_with_boundaries(map);

        self.position[0] = self.position[1];
        println!(
            "x = {}, y = {}, speed = {}, direction = {:?}, size = {:?}, time = {}",
            self.position[1].x, self.position[1].y, self.speed, self.direction, self.size, time
        );
    }

    fn check_collision(&self, /*plt: Vec2D, prb: Vec2D*/ position: Vec2D, size: Vec2D) -> bool {
        if (self.position[1].x - position.x).abs() > (self.size.x + size.x) / 2.0 {
            return false;
        }

        if (self.position[1].y - position.y).abs() > (self.size.y + size.y) / 2.0 {
            return false;
        }

        true
    }
}
