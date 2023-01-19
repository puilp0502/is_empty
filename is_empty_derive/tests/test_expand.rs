use is_empty_derive::IsEmpty;

mod is_empty {
    pub trait IsEmpty {
        fn is_empty(&self) -> bool;
    }

    impl<T> IsEmpty for Option<T> {
        fn is_empty(&self) -> bool {
            false
        }
    }
}

#[derive(IsEmpty)]
struct A {
    a: Option<u8>,
    #[is_empty(if = "Vec::is_empty")]
    b: Vec<u8>,
}
