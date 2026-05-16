use base::errors::ConfigError;
use base::parser;
use proc_macro::{self, Ident, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{DeriveInput, Visibility, parse_macro_input};

pub(crate) fn create_builder(ast: &DeriveInput) -> TokenStream2 {
    let name = format_ident!("{}Builder", ast.ident);

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("this is only supported by struct"),
    };

    let new_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();

        quote! {
            pub #ident: ::base::field::Field
        }
    });

    quote! {
        #[derive(Debug, Default)]
        pub struct #name {
            #( #new_fields, )*
        }
    }
}

pub(crate) fn impl_builder(ast: &DeriveInput) -> TokenStream2 {
    let struct_name = ast.ident.clone();
    let name = format_ident!("{}Builder", ast.ident);

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("this is only supported by struct"),
    };

    let field_names = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! {
            #ident
        }
    });

    let field_bldrs = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let stringify_ident = syn::LitStr::new(&ident.to_string(), ident.span());
        let ty = &f.ty;

        quote! {
            let #ident: #ty = self
                .#ident
                .parse::<#ty>()?;
        }
    });

    let methods = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let bldr_param = format_ident!("{}_bldr", ident);

        let fn_name = format_ident!("with_{}", ident);

        quote! {
            pub fn #fn_name(mut self, #bldr_param: ::base::field::FieldBuilder) -> Self {
                self.#ident = #bldr_param.build();
                self
            }
        }
    });

    quote! {
        impl #name {
            pub fn new() -> Self {
                Self::default()
            }

            #( #methods )*

            pub fn build(self) -> Result<#struct_name, ::base::errors::ConfigError> {
                #( #field_bldrs )*

                Ok(#struct_name {
                    #( #field_names, )*
                })
            }
        }
    }
}

pub(crate) fn loder_impl(ast: &DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    let builder = format_ident!("{}Builder", ident);
    let generics = quote! { <'a> };

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("this is only supported by struct"),
    };

    let field_bldrs = fields.iter().map(|f| {
        let field_ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let key = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

        quote! {
            let #field_ident = ::base::field::FieldBuilder::new(#key)
                .with_value(map.get(#key).map(|s| s.to_owned()));
        }
    });

    let methods = fields.iter().map(|f| {
        let field_ident = f.ident.as_ref().unwrap();
        let fn_name = format_ident!("with_{}", field_ident);
        let ty = &f.ty;
        let key = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

        quote! {
            .#fn_name(#field_ident)
        }
    });

    quote! {
        impl ::base::loader::ConfigLoader for #builder {
            type Out = #ident;

            fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError> {
                #( #field_bldrs )*

                #builder::new()
                    #( #methods )*
                    .build()
            }
        }

    }
}
