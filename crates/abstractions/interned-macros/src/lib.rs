use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn interned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use convert_case::{Case, Casing};

    let input = parse_macro_input!(item as DeriveInput);
    let vis = input.vis;
    let ty_ident = input.ident;
    let data_ty_ident = format_ident!("__{}Data", ty_ident);
    let storage_ident = format_ident!(
        "__{}_STORAGE",
        ty_ident.to_string().to_case(Case::UpperSnake)
    );

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the field definitions for both structs
    let field_defs = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });

    // Generate constructor parameters
    let ctor_params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });

    // Generate field initializers
    let field_inits = fields.iter().map(|f| {
        let field_ident = &f.ident;
        quote! { #field_ident }
    });

    let field_accesses = fields.iter().map(|f| {
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! {
            pub fn #field_ident(self) -> &'static #field_ty {
                &self.0.0.#field_ident
            }
        }
    });

    let expanded = quote! {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        #vis struct #data_ty_ident {
            #(#field_defs),*
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #vis struct #ty_ident(interned::Interned<#data_ty_ident>);

        interned::lazy_static! {
            static ref #storage_ident: std::sync::Mutex<interned::Storage<#data_ty_ident, 256>> =
                std::sync::Mutex::new(interned::Storage::default());
        }

        impl #ty_ident {
            #vis fn new(#(#ctor_params),*) -> Self {
                use interned::{lazy_static, Interned, Storage};
                use std::collections::HashSet;
                use std::sync::Mutex;

                let hidden = #data_ty_ident {
                    #(#field_inits),*
                };

                let mut storage = #storage_ident.lock().unwrap();
                #ty_ident(storage.intern(hidden))
            }

            #(#field_accesses)*
        }
    };

    TokenStream::from(expanded)
}
