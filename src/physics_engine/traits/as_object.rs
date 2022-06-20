use super::object_interface::ObjectInterface;

pub trait AsObject {
    fn as_object(&self) -> &dyn ObjectInterface;
}
