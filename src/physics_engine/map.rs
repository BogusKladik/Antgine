use super::{
    objects::line::Line,
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::vec2d::Vec2D,
    collision::Collision,
};

/// Structure describing the map on which objects exist and interact
pub struct Map {
    objects: Vec<Box<dyn ObjectInterface>>,
    pub dyn_objects: Vec<Box<dyn MoveInterface>>,
}

impl Map {
    /// Creating a map
    pub fn new() -> Map {
        Map {
            objects: Vec::<Box<dyn ObjectInterface>>::new(),
            dyn_objects: Vec::<Box<dyn MoveInterface>>::new(),
        }
    }

    /// Creates borders in the form of Lines
    pub fn init_map_border(&mut self, plt: Vec2D, prb: Vec2D) {
        self.objects.push(Box::new(Line::new(
            plt,
            Vec2D::new(prb.x, plt.y),
            0.0,
            1.0,
            Vec2D::default(),
            0.0,
            0.0,
            0.0,
        )));
        self.objects.push(Box::new(Line::new(
            Vec2D::new(prb.x, plt.y),
            prb,
            0.0,
            1.0,
            Vec2D::default(),
            0.0,
            0.0,
            0.0,
        )));
        self.objects.push(Box::new(Line::new(
            prb,
            Vec2D::new(plt.x, prb.y),
            0.0,
            1.0,
            Vec2D::default(),
            0.0,
            0.0,
            0.0,
        )));
        self.objects.push(Box::new(Line::new(
            Vec2D::new(plt.x, prb.y),
            plt,
            0.0,
            1.0,
            Vec2D::default(),
            0.0,
            0.0,
            0.0,
        )));
    }

    /// Start movement of objects belonging to this map
    pub fn run(&mut self, time: f32) {
        let mut collisions = Vec::<Collision>::new();

        // changes the potential characteristics of objects
        for i in 0..self.dyn_objects.len() {
            (*self.dyn_objects[i]).tracer(time);
        }

        // creates an array of collisions with non-moving objects, if any
        for i in 0..self.dyn_objects.len() {
            for j in 0..self.objects.len() {
                // checks circumscribed circles for collision
                if (*self.dyn_objects[i]).intersection_circumscribed_circles(&(*self.objects[j])) {
                    // checks for collision using the sat method
                    match (*self.dyn_objects[i]).sat(&(*self.objects[j])) {
                        Some((min_overlap, smallest_axis, contact_vertex)) => {
                            // TODO: here you need to fix the bug if you uncomment the code below. Without this line there will be no collision
                            // Error here
                            // Collision::new(Box::new(self.dyn_objects[i].as_object()), self.objects[j], min_overlap, smallest_axis, contact_vertex);
                        },
                        None => (),
                    }
                }
            }
        }

        // resolves collision
        while let Some(mut collision) = collisions.pop() {
            collision.divide_objects();
            collision.change_energy();
        }

        // creates an array of collisions with moving objects, if any
        for i in 0..self.dyn_objects.len() {
            for j in i..self.dyn_objects.len() {
                // checks circumscribed circles for collision
                if (*self.dyn_objects[i]).intersection_circumscribed_circles((*self.dyn_objects[j]).as_object()) {
                    // checks for collision using the sat method
                    match (*self.dyn_objects[i]).sat((*self.dyn_objects[j]).as_object()) {
                        Some((min_overlap, smallest_axis, contact_vertex)) => {
                            // TODO: here you need to fix the bug if you uncomment the code below. Without this line there will be no collision
                            // Error here
                            // Collision::new(Box::new(self.dyn_objects[i].as_object()), self.objects[j], min_overlap, smallest_axis, contact_vertex);
                        },
                        None => (),
                    }
                }
            }
        }

        // changes the potential characteristics of objects
        while let Some(mut collision) = collisions.pop() {
            collision.divide_objects();
            collision.change_energy();
        }

        // assigns potential characteristics to present characteristics
        for i in 0..self.dyn_objects.len() {
            (*self.dyn_objects[i]).run(time);
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}
