use proc_macro::{self, Ident, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Visibility, parse_macro_input};

pub(crate) fn create_builder(ast: &DeriveInput) -> TokenStream2 {
    let name = format_ident!("{}Builder", ast.ident);
    let generics = quote! { <'a> };

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("this is only supported by struct"),
    };

    let new_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        quote! {
            #ident: base::field::Field<'a, #ty>
        }
    });

    quote! {
        #[derive(Debug, Default)]
        pub struct #name #generics {
            #( #new_fields, )*
        }
    }
}

pub(crate) fn impl_builder(ast: &DeriveInput) -> TokenStream2 {
    let struct_name = ast.ident.clone();
    let name = format_ident!("{}Builder", ast.ident);
    let generics = quote! { <'a> };

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("this is only supported by struct"),
    };

    let new_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        quote! {
            #ident: self.#ident.value()
        }
    });

    let methods = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let key = syn::LitStr::new(&ident.to_string(), ident.span());

        let fn_name = format_ident!("with_{}", ident);

        quote! {
            pub fn #fn_name<S: Into<#ty>>(mut self, #ident: S) -> Self {
                self.#ident = base::field::Field::new(#key, #ident.into());
                self
            }
        }
    });

    quote! {
        impl #generics #name #generics {
            pub fn new() -> Self {
                Self::default()
            }

            #( #methods )*

            pub fn build(self) -> Result<#struct_name, ConfigError> {
                Ok(#struct_name {
                     #( #new_fields, )*
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

    let new_fields = fields.iter().map(|f| {
        let field_ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let key = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

        quote! {
            let #field_ident = base::field::FieldBuilder::new(#key)
                .with_value(required_str(&map, #key)?)
                .build::<#ty>()?;
        }
    });

    let methods = fields.iter().map(|f| {
        let field_ident = f.ident.as_ref().unwrap();
        let fn_name = format_ident!("with_{}", field_ident);
        let ty = &f.ty;
        let key = syn::LitStr::new(&field_ident.to_string(), field_ident.span());

        quote! {
            .#fn_name(#field_ident.value())
        }
    });

    quote! {
        impl base::loader::ConfigLoader for #builder <'_> {
            type Out = #ident;

            fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError> {
                #( #new_fields )*

                #builder::new()
                    #( #methods )*
                    .build()
            }
        }

    }
}
