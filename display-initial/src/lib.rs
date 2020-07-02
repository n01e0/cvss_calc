extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

/// `format!("{}", hoge)`した時に, `{:?}`のイニシャルを出す
#[proc_macro_derive(DisplayInitial)]
pub fn derive_display_initial(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);
    let enum_name = item.ident;

    let gen = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", format!("{:?}", self).chars().next().unwrap())
            }
        }
    };
    gen.into()
}
