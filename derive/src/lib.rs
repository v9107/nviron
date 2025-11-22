#![allow(unused)]

use proc_macro::{self, Ident, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Visibility, parse_macro_input};

mod helpers;

#[proc_macro_derive(EnvBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = &derive_input;

    let name = format_ident!("{}Builder", ident);
    let struct_builder = helpers::create_builder(&derive_input);
    let implementation = helpers::impl_builder(&derive_input);
    let trait_impl = helpers::loder_impl(&derive_input);
    let output = quote! {

        #struct_builder

        #implementation

        #trait_impl

    };
    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
