extern crate proc_macro;
use proc_macro::TokenStream;
use std::env;
use syn::{Error, Item};
use quote::{quote, ToTokens};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn cxx_bindgen(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = match syn::parse2::<Item>(input.into()) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error().into(),
    };
    let ops = match syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated.parse(args) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = match item {
        Item::Struct(_) | Item::Enum(_) | Item::Fn(_) | Item::Impl(_) => {
            let meta_attr = if env::var("CXX_BINDGEN_RUNNING").is_ok_and(|x| x == env::var("CARGO_PKG_NAME").unwrap()) {
                quote! { #[cxx_bindgen::cxx_bindgen_meta(#ops)] }
            } else {
                quote! {}
            };

            Ok(TokenStream::from(quote! {
                #meta_attr
                #item
            }))
        }
        _ => {
            Err(Error::new_spanned(item.to_token_stream(), "#[cxx_bindgen] can only be used on structs, enums, functions or impl blocks"))
        }
    };

    expanded
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}