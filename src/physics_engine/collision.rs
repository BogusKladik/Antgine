use super::{traits::object_interface::ObjectInterface, types::vec2d::Vec2D};

/// A structure that solves the collision of two objects
pub struct Collision {
    object1: Box<dyn ObjectInterface>,
    object2: Box<dyn ObjectInterface>,
    min_overlap: f32,
    smallest_axis: Vec2D,
    contact_vertex: Vec2D,
}

impl Collision {
    /// Creating a structure with information about a collision
    pub fn new(
        object1: Box<dyn ObjectInterface>,
        object2: Box<dyn ObjectInterface>,
        min_overlap: f32,
        smallest_axis: Vec2D,
        contact_vertex: Vec2D,
    ) -> Self {
        Collision {
            object1,
            object2,
            min_overlap,
            smallest_axis,
            contact_vertex,
        }
    }

    /// Modifies object properties to separate objects by smallest asix
    pub fn divide_objects(&mut self) {
        // separates 2 objects along the smallest axis
        let divide_objects = self.smallest_axis.mul_n(
            self.min_overlap
                / (self.object1.get_inversion_mass() + self.object2.get_inversion_mass()),
        );

        // pushes 1 object away
        self.object1.set_current_position(
            self.object1.get_current_position()
                + divide_objects.mul_n(self.object1.get_inversion_mass()),
        );
        // pushes 2 object away
        self.object2.set_current_position(
            self.object2.get_current_position()
                + divide_objects.mul_n(-self.object2.get_inversion_mass()),
        );
    }

    /// Simulates the collision of two objects , changing their properties
    pub fn change_energy(&mut self) {
        // calculates directions after collision, speeds after separation
        let collision_arm1 = self.contact_vertex - self.object1.get_potential_position();
        let rotation_velocity1 = Vec2D::new(
            -self.object1.get_angle_velocity() * collision_arm1.y,
            self.object1.get_angle_velocity() * collision_arm1.x,
        );
        let closing_velocity1 = self.object1.get_velocity() + rotation_velocity1;
        let collision_arm2 = self.contact_vertex - self.object2.get_potential_position();
        let rotation_velocity2 = Vec2D::new(
            -self.object2.get_angle_velocity() * collision_arm2.y,
            self.object2.get_angle_velocity() * collision_arm2.x,
        );
        let closing_velocity2 = self.object2.get_velocity() + rotation_velocity2;
        
        // calculates the momentum acceleration
        let mut impulse_augmentation1 = Vec2D::cross(&collision_arm1, &self.smallest_axis);
        impulse_augmentation1 =
            impulse_augmentation1 * self.object1.get_inversion_inertia() * impulse_augmentation1;
        let mut impulse_augmentation2 = Vec2D::cross(&collision_arm2, &self.smallest_axis);
        impulse_augmentation2 =
            impulse_augmentation2 * self.object2.get_inversion_inertia() * impulse_augmentation2;

        // calculates velocity after collision
        let relative_velocity = closing_velocity1 - closing_velocity2;
        let separate_velocity = Vec2D::dot(&relative_velocity, &self.smallest_axis);
        let new_separate_velocity = -separate_velocity
            * self
                .object1
                .get_elasticity()
                .min(self.object2.get_elasticity());
        let separate_velocity_difference = new_separate_velocity - separate_velocity;

        // calculates the momentum to further change characteristics
        let impulse = separate_velocity_difference
            / (self.object1.get_inversion_mass()
                + self.object2.get_inversion_mass()
                + impulse_augmentation1
                + impulse_augmentation2);
        let impulse_vector = self.smallest_axis.mul_n(impulse);

        // changes the velocity of 1 object
        self.object1.set_velocity(
            self.object1.get_velocity() + impulse_vector.mul_n(self.object1.get_inversion_mass()),
        );
        // changes the velocity of 2 object
        self.object2.set_velocity(
            self.object2.get_velocity() + impulse_vector.mul_n(-self.object2.get_inversion_mass()),
        );

        // changes the angular velocity of 1 object
        self.object1.set_angle_velocity(
            self.object1.get_angle_velocity()
                + self.object1.get_inversion_inertia()
                    * Vec2D::cross(&collision_arm1, &impulse_vector),
        );
        // changes the angular velocity of 2 object
        self.object2.set_angle_velocity(
            self.object2.get_angle_velocity()
                - self.object2.get_inversion_inertia()
                    * Vec2D::cross(&collision_arm2, &impulse_vector),
        );
    }
}
