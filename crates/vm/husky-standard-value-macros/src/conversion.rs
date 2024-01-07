mod r#enum;
mod r#struct;

use husky_macro_utils::{self_ty};
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn value_conversion(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    match item {
        syn::Item::Enum(item) => r#enum::enum_value_conversion(item),
        syn::Item::Struct(item) => r#struct::struct_value_conversion(item),
        _ => todo!(),
    }
}

fn impl_weak_static_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                #param: __WeakStatic<Static = #param> + __Static + __Serialize
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_weak_static_associated_ty_static(
    ident: &Ident,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __WeakStatic>::Static })
                }
                syn::GenericParam::Lifetime(_param) => None,
                syn::GenericParam::Const(param) => {
                    let ident = &param.ident;
                    Some(quote! { #ident })
                }
            }),
        );
        quote! { #ident<#arguments> }
    }
}

fn impl_static_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(param) => quote! {
                #param: 'static,
            },
            syn::GenericParam::Type(param) => quote! {
                #param: __Static + __Serialize
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_static_associated_ty_frozen(
    ident: &Ident,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __Static>::Frozen })
                }
                syn::GenericParam::Lifetime(_param) => None,
                syn::GenericParam::Const(param) => {
                    let ident = &param.ident;
                    Some(quote! { #ident })
                }
            }),
        );
        quote! { #ident<#arguments> }
    }
}

fn impl_frozen_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(param) => quote! {
                #param: 'static,
            },
            syn::GenericParam::Type(param) => quote! {
                #param: __Frozen
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_frozen_associated_ty_static(
    ident: &Ident,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __Frozen>::Static })
                }
                syn::GenericParam::Lifetime(_param) => None,
                syn::GenericParam::Const(param) => {
                    let ident = &param.ident;
                    Some(quote! { #ident })
                }
            }),
        );
        quote! { #ident<#arguments> }
    }
}

fn impl_from_value_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                // ad hoc
                #param: __WeakStatic<Static = #param> + __Static
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_into_value_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                // ad hoc
                #param: __WeakStatic<Static = #param> + __Static
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}
