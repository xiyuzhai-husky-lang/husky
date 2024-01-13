use husky_macro_utils::self_ty;
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
    let self_ty = self_ty(ty_ident, generics);
    if fields.unnamed.len() == 0 {
        return None;
    } else if fields.unnamed.len() == 1 {
        let field_ty = &fields.unnamed[0].ty;
        // todo: generics
        Some(quote! {
            impl #generics From<#field_ty> for #self_ty {
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
                        impl #generics From<#last_field_ty> for #self_ty {
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
