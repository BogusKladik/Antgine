use super::ObjectInterface::ObjectInterface;

pub trait AsObject {
    fn as_object(&self) -> &dyn ObjectInterface;
}
