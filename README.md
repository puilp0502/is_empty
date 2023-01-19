# is_empty
Easily check if the struct is empty.

## Usage

```rust
use is_empty::IsEmpty;

#[derive(IsEmpty)]
struct Foo {
    a: Option<String>,
    b: Option<String>,
}

let foo = Foo {
    a: Some("a".to_string()),
    b: None,
};
assert!(!foo.is_empty());

let bar = Foo {
   a: None,
   b: None,
};
assert!(bar.is_empty());
```

### Acknowledgements

This crate was made possible by the excellent [blog series](https://blog.turbo.fish/proc-macro-simple-derive/) on derive macros by Jonas Platte.  

### License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.  

### Contribution 

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
