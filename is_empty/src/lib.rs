pub use is_empty_derive::*;

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for std::option::Option<T> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}
