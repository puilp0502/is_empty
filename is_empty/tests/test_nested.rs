#![allow(dead_code)]
use is_empty::IsEmpty;

#[derive(IsEmpty)]
struct Foo {
    bar: Bar,
}

#[derive(IsEmpty)]
struct Bar {
    c: Option<u8>,
    #[is_empty(if = "Vec::is_empty")]
    d: Vec<u8>,
}

#[test]
fn test_is_empty_nested() {
    let foo = Foo {
        bar: Bar { c: None, d: vec![] },
    };
    assert!(foo.is_empty());
    let foo = Foo {
        bar: Bar {
            c: Some(1),
            d: vec![],
        },
    };
    assert!(!foo.is_empty());
    let foo = Foo {
        bar: Bar {
            c: None,
            d: vec![1],
        },
    };
    assert!(!foo.is_empty());
}
