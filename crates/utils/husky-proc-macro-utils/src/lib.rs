//! useful things for creating procedural macros
use quote::quote;

pub fn generics_with_debug_with_db(generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! {}
    } else {
        let lifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(_) | syn::GenericParam::Const(_) => None,
                    syn::GenericParam::Lifetime(_) => Some(quote! { #param }),
                }),
            );
        let nonlifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(param) => Some(if param.bounds.is_empty() {
                        quote! { #param: ::salsa::DebugWithDb }
                    } else {
                        quote! { #param + ::salsa::DebugWithDb }
                    }),
                    syn::GenericParam::Const(_) => Some(quote! { #param }),
                    syn::GenericParam::Lifetime(_) => None,
                }),
            );
        if lifetime_generics_punctuated.is_empty() {
            if nonlifetime_generics_punctuated.is_empty() {
                quote! {}
            } else {
                quote! { <#nonlifetime_generics_punctuated> }
            }
        } else {
            quote! { <#lifetime_generics_punctuated, #nonlifetime_generics_punctuated> }
        }
    }
}

pub fn generics_without_bounds(generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! {}
    } else {
        let lifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(_) | syn::GenericParam::Const(_) => None,
                    syn::GenericParam::Lifetime(param) => {
                        let lifetime = &param.lifetime;
                        Some(quote! { #lifetime })
                    }
                }),
            );
        let nonlifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(param) => {
                        let param_ident = &param.ident;
                        Some(quote! { #param_ident })
                    }
                    syn::GenericParam::Const(param) => {
                        let param_ident = &param.ident;
                        Some(quote! { #param_ident })
                    }
                    syn::GenericParam::Lifetime(_) => None,
                }),
            );
        if lifetime_generics_punctuated.is_empty() {
            if nonlifetime_generics_punctuated.is_empty() {
                quote! {}
            } else {
                quote! { <#nonlifetime_generics_punctuated> }
            }
        } else {
            quote! { <#lifetime_generics_punctuated, #nonlifetime_generics_punctuated> }
        }
    }
}

pub fn self_ty(ident: &proc_macro2::Ident, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    quote! { #ident }
                }
                syn::GenericParam::Lifetime(param) => {
                    let lifetime = &param.lifetime;
                    quote! { #lifetime }
                }
                syn::GenericParam::Const(param) => {
                    let ident = &param.ident;
                    quote! { #ident }
                }
            }),
        );
        quote! { #ident<#arguments> }
    }
}
