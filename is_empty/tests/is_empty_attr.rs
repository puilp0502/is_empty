#![allow(dead_code)]
use is_empty::IsEmpty;

#[derive(IsEmpty)]
struct Foo {
    a: Option<u8>,
    #[is_empty(if = "Vec::is_empty")]
    b: Vec<u8>,
}

#[test]
fn test_is_empty_for_vec_with_attr() {
    let foo = Foo { a: None, b: vec![] };
    assert!(foo.is_empty());
    let bar = Foo {
        a: Some(1),
        b: vec![],
    };
    assert!(!bar.is_empty());
}
