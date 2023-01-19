use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, ExprPath, Fields, Ident, Lit, Meta, NestedMeta,
};

fn get_is_empty_if_attr(attr: &Attribute) -> syn::Result<Option<ExprPath>> {
    let meta = attr.parse_meta()?;
    let meta_list = match meta {
        Meta::List(meta_list) => meta_list,
        _ => {
            return Err(syn::Error::new_spanned(
                meta,
                "expected a list-style attribute",
            ))
        }
    };
    let nested = match meta_list.nested.len() {
        0 => return Ok(None),
        1 => &meta_list.nested[0],
        _ => return Err(Error::new_spanned(meta_list, "expected a single attribute")),
    };

    // If the attribute looks like #[is_empty(if = "Vec::is_empty")], then the name_value is
    // path: "if"
    // lit: "Vec::is_empty"
    let name_value = match nested {
        NestedMeta::Meta(Meta::NameValue(nv)) => nv,
        _ => {
            return Err(Error::new_spanned(
                nested,
                "expected a name-value attribute",
            ))
        }
    };

    if !name_value.path.is_ident("if") {
        return Err(Error::new_spanned(name_value, "expected an `if` attribute"));
    }

    match &name_value.lit {
        Lit::Str(lit_str) => {
            let ident = lit_str.value();
            let ident = syn::parse_str(&ident)?;
            Ok(Some(ident))
        }
        _ => Err(Error::new_spanned(name_value, "expected a string literal")),
    }
}

pub fn expand_is_empty(input: DeriveInput) -> syn::Result<TokenStream> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("derive(IsEmpty) only works on structs"),
    };

    let determinants = fields
        .into_iter()
        .map(|f| {
            let attrs: Vec<_> = f
                .attrs
                .iter()
                // This ensures that we only work with #[is_empty] attributes
                .filter(|a| a.path.is_ident("is_empty"))
                .collect();

            let determinant_from_attr = match attrs.len() {
                0 => None,
                1 => get_is_empty_if_attr(attrs[0])?,
                _ => {
                    let mut error = Error::new_spanned(attrs[1], "redundant `is_empty` attribute");
                    error.combine(Error::new_spanned(
                        attrs[0],
                        "previous `is_empty` attribute",
                    ));
                    return Err(error);
                }
            };

            let method = determinant_from_attr
                .unwrap_or(syn::parse_str::<ExprPath>("is_empty::IsEmpty::is_empty")?);
            let field_ident = f.ident;
            Ok(quote! {
                if !#method ( &self.#field_ident ) { return false };
            })
        })
        .collect::<syn::Result<TokenStream>>()?;
    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics is_empty::IsEmpty for #st_name #ty_generics #where_clause {
            fn is_empty(&self) -> bool {
                #determinants
                true
            }
        }
    })
}
