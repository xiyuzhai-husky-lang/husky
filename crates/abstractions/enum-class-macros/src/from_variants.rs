use proc_macro2::Ident;
use syn::{Field, FieldsUnnamed, Generics, Item, ItemEnum, Type};

pub(crate) fn from_variants(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as Item);
    let from_variant_impls = match item {
        Item::Enum(ref item) => enum_from_variant_impls(&item),
        _ => panic!("expect enum for `from_variants`"),
    };
    quote! {
        #item

        #from_variant_impls
    }
    .into()
}

fn enum_from_variant_impls(item: &ItemEnum) -> proc_macro2::TokenStream {
    let ty_ident = &item.ident;

    item.variants
        .iter()
        .filter_map(|variant| -> Option<proc_macro2::TokenStream> {
            match variant.fields {
                syn::Fields::Unnamed(ref fields) => {
                    enum_from_variant_impl(&item.generics, ty_ident, &variant.ident, fields)
                }
                _ => None,
            }
        })
        .collect::<proc_macro2::TokenStream>()
}

fn enum_from_variant_impl(
    generics: &Generics,
    ty_ident: &Ident,
    variant_ident: &Ident,
    fields: &FieldsUnnamed,
) -> Option<proc_macro2::TokenStream> {
    if fields.unnamed.len() == 1 {
        let field_ty = &fields.unnamed[0].ty;
        // todo: generics
        Some(quote! {
            impl #generics From<#field_ty> for #ty_ident #generics {
                fn from(value: #field_ty) -> Self {
                    #ty_ident::#variant_ident(value)
                }
            }
        })
    } else {
        let Type::Path(ref ty_path) = fields.unnamed[0].ty else {
            return None;
        };
        match ty_path.path.get_ident() {
            Some(ty_path_ident) => {
                if ty_path_ident == "Room32" {
                    let last_field_ty = fields.unnamed.last().unwrap();
                    let defaults: proc_macro2::TokenStream = (0..(fields.unnamed.len() - 1))
                        .into_iter()
                        .map(|_| quote! { Default::default(), })
                        .collect();
                    Some(quote! {
                        impl #generics From<#last_field_ty> for #ty_ident #generics {
                            fn from(value: #last_field_ty) -> Self {
                                #ty_ident::#variant_ident(#defaults value)
                            }
                        }
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

pub(crate) fn const_from_variants(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as Item);
    let const_from_variant_impls = match item {
        Item::Enum(ref item) => enum_const_from_variant_impls(&item),
        _ => panic!("expect enum for `from_variants`"),
    };
    quote! {
        #item

        #const_from_variant_impls
    }
    .into()
}

fn enum_const_from_variant_impls(item: &ItemEnum) -> proc_macro2::TokenStream {
    let ty_ident = &item.ident;

    item.variants
        .iter()
        .filter_map(|variant| -> Option<proc_macro2::TokenStream> {
            if variant.fields.len() != 1 {
                return None;
            }
            match variant.fields {
                syn::Fields::Unnamed(_) => Some(enum_const_from_variant_impl(
                    ty_ident,
                    &variant.ident,
                    variant.fields.iter().next().unwrap(),
                )),
                _ => None,
            }
        })
        .collect::<proc_macro2::TokenStream>()
}

fn enum_const_from_variant_impl(
    ty_ident: &Ident,
    variant_ident: &Ident,
    field: &Field,
) -> proc_macro2::TokenStream {
    let field_ty = &field.ty;
    // todo: generics
    quote! {
        impl From<#field_ty> for #ty_ident {
            fn from(value: #field_ty) -> Self {
                #ty_ident::#variant_ident(value)
            }
        }
    }
}
