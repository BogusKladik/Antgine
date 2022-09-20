use super::object_interface::ObjectInterface;

/// Trait to convert supertrait to trait ObjectInterface
pub trait AsObject {
    /// Returns the converted supertrait to trait ObjectInterface
    fn as_object(&self) -> &dyn ObjectInterface;
}
