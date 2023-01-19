pub use derive_is_empty::*;

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for std::option::Option<T> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}
