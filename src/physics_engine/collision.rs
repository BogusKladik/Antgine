use super::traits::ObjectInterface;

use super::vec2d::Vec2D;

struct Collision {
    object1: &dyn ObjectInterface,
    object2: &dyn ObjectInterface,
    min_overlap: f32,
    smallest_axis: Vec2D,
    contact_vertex: Vec2D,
}

impl Collision {
    pub fn new(
        object1: &dyn ObjectInterface,
        object2: &dyn ObjectInterface,
        (min_overlap, smallest_axis, contact_vertex): (f32, Vec2D, Vec2D),
    ) -> Self {
        Collision {
            object1,
            object2,
            min_overlap,
            smallest_axis,
            contact_vertex,
        }
    }

    pub fn divide_objects() {
        todo!()
    }

    pub fn change_energy() {
        todo!()
    }
}