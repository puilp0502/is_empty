mod is_empty;
use is_empty::expand_is_empty;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IsEmpty, attributes(is_empty))]
pub fn derive_is_empty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_is_empty(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
