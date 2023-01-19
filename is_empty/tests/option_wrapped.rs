use is_empty::{is_option_really_empty, IsEmpty};

#[derive(IsEmpty)]
struct Foo {
    a: Option<u8>,
}

#[derive(IsEmpty)]
struct RootWithCheck {
    #[is_empty(if = "is_empty::is_option_really_empty")]
    foo: Option<Foo>,
}

#[test]
fn test_option_wrapped() {
    let empty_root = RootWithCheck { foo: None };
    assert!(empty_root.is_empty());

    let empty_root = RootWithCheck {
        foo: Some(Foo { a: None }),
    };
    assert!(empty_root.is_empty());

    let empty_root = RootWithCheck {
        foo: Some(Foo { a: Some(1) }),
    };
    assert!(!empty_root.is_empty());
}
