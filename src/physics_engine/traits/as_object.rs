use super::object_interface::ObjectInterface;

/// Trait to convert supertrait to trait ObjectInterface
pub trait AsObject {
    /// Returns the converted supertrait to trait ObjectInterface
    fn as_object(&self) -> &dyn ObjectInterface;
    fn as_mut_object(&mut self) -> &mut dyn ObjectInterface;
}
