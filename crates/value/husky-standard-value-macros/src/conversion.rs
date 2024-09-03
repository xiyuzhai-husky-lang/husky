mod r#enum;
mod r#struct;

use husky_proc_macro_utils::self_ty;
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

fn impl_immortal_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                #param: __Immortal + __Serialize
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_boiled_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                #param: __Boiled<Thawed = #param> + __Thawed + __Serialize
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_boiled_assoc_thawed(ident: &Ident, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __Boiled>::Thawed })
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

fn impl_thawed_generic_constraints(generics: &syn::Generics) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(param) => quote! {
                #param: 'static,
            },
            syn::GenericParam::Type(param) => quote! {
                #param: __Thawed + __Serialize
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_thawed_assoc_frozen(ident: &Ident, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __Thawed>::Frozen })
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

fn impl_frozen_assoc_thawed(ident: &Ident, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            generics.params.iter().filter_map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    Some(quote! { <#ident as __Frozen>::Thawed })
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
                #param: __Immortal
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
                #param: __Immortal
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_from_thawed_value_generic_constraints(
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                // ad hoc
                #param: __Boiled<Thawed = #param> + __Thawed
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}

fn impl_into_thawed_value_generic_constraints(
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_param) => quote! {},
            syn::GenericParam::Type(param) => quote! {
                // ad hoc
                #param: __Boiled<Thawed = #param> + __Thawed
            },
            syn::GenericParam::Const(_) => quote! {},
        })
        .collect::<proc_macro2::TokenStream>()
}
