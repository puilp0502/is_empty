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
}
