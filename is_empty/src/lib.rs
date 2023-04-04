//! Easily check if the struct is empty.  
//!
//! Say you have a struct full of `Option<T>` fields, and you want to know if all the fields are `None`.
//! ```
//! struct Foo {
//!     client_ip: Option<String>,
//!     client_country: Option<String>,
//! }
//! ```
//! You can manually check for each field like this:
//! ```
//!# struct Foo {
//!#     client_ip: Option<String>,
//!#     client_country: Option<String>,
//!# }
//! impl Foo {
//!     fn is_empty(&self) -> bool {
//!        self.client_ip.is_none() && self.client_country.is_none()
//!     }
//! }
//! ```
//! But this becomes tedious as more and more fields are added to the struct.
//!
//! With this crate, you can derive the `IsEmpty` trait, and then call is_empty() on the struct.
//! ```
//! use is_empty::IsEmpty;
//!
//! #[derive(IsEmpty)]
//! struct Foo {
//!     client_ip: Option<String>,
//!     client_country: Option<String>,
//! }
//!
//! let empty_foo = Foo { client_ip: None, client_country: None };
//! assert!(empty_foo.is_empty());
//! ```
//!
//! You can also nest other `IsEmpty`-deriving struct inside the struct.
//! ```
//! use is_empty::IsEmpty;
//!
//! #[derive(IsEmpty)]
//! struct Foo {
//!     bar: Bar,
//!     baz: Option<u8>
//! }
//! #[derive(IsEmpty)]
//! struct Bar {
//!     client_ip: Option<String>,
//!     client_country: Option<String>,
//! }
//!
//! let empty_foo = Foo { bar: Bar { client_ip: None, client_country: None }, baz: None };
//! assert!(empty_foo.is_empty());
//! ```
//!
//! If you want to customize the logic for determining if the field is empty, you can use the `#[is_empty(if = "some_fn")]` attribute.
//! ```
//! use is_empty::IsEmpty;
//!
//! #[derive(IsEmpty)]
//! struct Foo {
//!     #[is_empty(if = "Vec::is_empty")]
//!     bar: Vec<u8>,
//! }
//!
//! let empty_foo = Foo { bar: vec![] };
//! assert!(empty_foo.is_empty());
//! ```
//!
//! This crate pairs well with serde's `#[serde(skip_serializing_if = "condition")]` attribute.
//! ```
//! use is_empty::IsEmpty;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, IsEmpty)]
//! struct Foo {
//!     client_ip: Option<String>,
//!     client_country: Option<String>,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Root {
//!     #[serde(skip_serializing_if = "is_empty::is_empty")]
//!     foo: Foo,
//! }
//!
//! let empty_foo = Foo { client_ip: None, client_country: None };
//! let root = Root { foo: empty_foo };
//! assert_eq!(serde_json::to_string(&root).unwrap(), "{}");
//! ```
//!
pub use is_empty_derive::*;

#[cfg(feature = "std_impls")]
pub mod std_impls;

/// A trait for checking if a struct is empty.
/// See the [crate-level documentation](crate) for more information.
pub trait IsEmpty {
    /// Check if the struct is empty.
    /// Returns true if all the fields are considered empty.
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for std::option::Option<T> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

/// Thin wrapper function around [IsEmpty::is_empty]. Use it with serde's skip_serializing_if attribute.
/// ```
/// use is_empty::IsEmpty;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, IsEmpty)]
/// struct Foo {
///     client_ip: Option<String>,
///     client_country: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize)]
/// struct Root {
///     #[serde(skip_serializing_if = "is_empty::is_empty")]
///     foo: Foo,
/// }
///
/// let empty_foo = Foo { client_ip: None, client_country: None };
/// let root = Root { foo: empty_foo };
/// assert_eq!(serde_json::to_string(&root).unwrap(), "{}");
/// ```
pub fn is_empty<T>(t: &T) -> bool
where
    T: IsEmpty,
{
    t.is_empty()
}

/// Check if the `Option<T>` is really empty.  
///
/// [is_empty] returns false for `Some(T)`, even if `T` is empty.
/// This function inspects the struct wrapped inside `Option::Some<T>`, and returns true if it is empty.
/// (Of course, it returns true for `None`.)
///
/// This function can be used in two ways:
/// - in a `#[is_empty(if = "is_empty::is_option_really_empty")]` attribute,
/// - in a `#[serde(skip_serializing_if = "is_empty::is_option_really_empty")]` attribute.
pub fn is_option_really_empty<T>(t: &Option<T>) -> bool
where
    T: IsEmpty,
{
    match t {
        None => true,
        Some(t) => t.is_empty(),
    }
}
