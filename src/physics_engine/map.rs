use super::{
    objects::{line::Line, rectangle::Rectangle},
    traits::{move_interface::MoveInterface, object_interface::ObjectInterface},
    types::{vec2d::Vec2D, angle::Angle},
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

    pub fn test_rectangle(&mut self) {
        self.objects.push(Box::new(Rectangle::new(
            Vec2D::new(1920.0, 0.0),
            Vec2D::new(1920.0, 20.0),
            20.0,
            0.0,
            1.0,
            Vec2D::default(),
            0.0,
            0.0,
            0.0
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
                    if let Some((min_overlap, smallest_axis, contact_vertex)) = (*self.dyn_objects[i]).sat(&(*self.objects[j])) {
                        let mut collision = Collision::new(self.dyn_objects[i].as_mut_object(), self.objects[j].as_mut_object(), min_overlap, smallest_axis, contact_vertex);
                        collision.divide_objects();
                        collision.change_energy()
                    }
                }
            }
        }

        // TODO: Either delete the array, or increase the scope of Collision
        // resolves collision
        while let Some(mut collision) = collisions.pop() {
            collision.divide_objects();
            collision.change_energy();
        }

        // creates an array of collisions with moving objects, if any
        for i in 0..self.dyn_objects.len() {
            let (l_dyn_objects, r_dyn_objects) = self.dyn_objects.split_at_mut(i + 1);
            for r_dyn_object in r_dyn_objects.iter_mut() {
                // checks circumscribed circles for collision
                if l_dyn_objects[i].intersection_circumscribed_circles(r_dyn_object.as_object()) {
                    // checks for collision using the sat method
                    if let Some((min_overlap, smallest_axis, contact_vertex)) = l_dyn_objects[i].sat(r_dyn_object.as_object()) {
                        let mut collision = Collision::new(l_dyn_objects[i].as_mut_object(), r_dyn_object.as_mut_object(), min_overlap, smallest_axis, contact_vertex);
                        collision.divide_objects();
                        collision.change_energy()
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

    pub fn create_data(&self) -> (Vec<Vec2D>, Vec<Angle>) {
        let mut pos = Vec::with_capacity(self.dyn_objects.len() + self.objects.len());
        for dyn_object in &self.dyn_objects {
            pos.push(dyn_object.get_current_position());
        }

        for object in &self.objects {
            pos.push(object.get_current_position());
        }

        let mut ang = Vec::with_capacity(self.dyn_objects.len() + self.objects.len());
        for dyn_object in &self.dyn_objects {
            ang.push(dyn_object.get_angle());
        }

        for object in &self.objects {
            ang.push(object.get_angle());
        }

        (pos, ang)
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}
