pub use derive_is_empty::*;

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}
