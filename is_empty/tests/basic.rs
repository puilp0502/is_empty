#![allow(dead_code)]
use is_empty::IsEmpty;

#[derive(IsEmpty)]
struct Foo {
    a: Option<u8>,
    b: Option<u8>,
}

#[test]
fn test_is_empty() {
    let foo = Foo { a: None, b: None };
    assert!(foo.is_empty());
    let bar = Foo {
        a: Some(1),
        b: None,
    };
    assert!(!bar.is_empty());
    let baz = Foo {
        a: None,
        b: Some(1),
    };
    assert!(!baz.is_empty());
    let qux = Foo {
        a: Some(1),
        b: Some(1),
    };
    assert!(!qux.is_empty());
}
