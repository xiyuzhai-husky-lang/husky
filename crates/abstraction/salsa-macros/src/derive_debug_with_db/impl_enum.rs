use syn::{ItemEnum, Variant};

use super::*;

pub(super) fn enum_debug_with_db_impl(jar_ty: &Type, item: &ItemEnum) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let ident_string = ident.to_string();

    let variants = item
        .variants
        .iter()
        .map(|variant| -> proc_macro2::TokenStream {
            match variant.fields {
                syn::Fields::Named(_) => enum_struct_variant_debug_with_db(jar_ty, ident, variant),
                syn::Fields::Unnamed(_) => enum_tuple_variant_debug_with_db(jar_ty, ident, variant),
                syn::Fields::Unit => enum_unit_variant_debug_with_db(jar_ty, ident, variant),
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        impl<DB: ::salsa::DbWithJar<#jar_ty> + ?Sized> ::salsa::DebugWithDb<DB> for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &DB, _include_all_fields: bool) -> ::std::fmt::Result {
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                match self {
                    #variants
                }
            }
        }
    }
}

fn enum_struct_variant_debug_with_db(
    jar_ty: &Type,
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let ident_string = variant_ident.to_string();
    let field_decls = variant
        .fields
        .iter()
        .map(|field| -> proc_macro2::TokenStream {
            let field_ident = field.ident.as_ref().unwrap();
            quote_spanned! { field.span() =>
                ref #field_ident
            }
        });
    let field_debugs = variant
        .fields
        .iter()
        .map(|field| -> proc_macro2::TokenStream {
            let field_ident = &field.ident.as_ref().unwrap();
            let field_ident_string = field_ident.to_string();
            let field_ty = &field.ty;
            // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
            let field_debug = quote_spanned! { field.span() =>
                debug_struct = debug_struct.field(
                    #field_ident_string,
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, DB>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        #field_ident,
                        _db,
                        _include_all_fields
                    )
                );
            };

            quote_spanned! { field.span() =>
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
            #ty_ident::#variant_ident { #(#field_decls),* }  => {
                let mut debug_struct = &mut f.debug_struct(#ident_string);

                #field_debugs

                debug_struct.finish()
            }
    }
}

fn enum_tuple_variant_debug_with_db(
    jar_ty: &Type,
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let name_field = |field_idx: usize| format!("v{}", field_idx);
    let field_decls =
        variant
            .fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| -> proc_macro2::TokenStream {
                let field_ident = Ident::new(&name_field(field_idx), field.span());
                quote_spanned! { field.span() =>
                    ref #field_ident
                }
            });
    let field_debugs = variant
        .fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
            let field_ident = Ident::new(&name_field(field_idx), field.span());
            let field_ty = &field.ty;

            let field_debug = quote_spanned! { field.span() =>
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, DB>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &#field_ident,
                        _db,
                        _include_all_fields
                    )
                );
            };

            quote_spanned! { field.span() =>
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
            #ty_ident::#ident(#(#field_decls),*) => {
                let mut debug_tuple = &mut f.debug_tuple(#ident_string);

                #field_debugs

                debug_tuple.finish()
            }
    }
}

fn enum_unit_variant_debug_with_db(
    jar_ty: &Type,
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let ident_string = ident.to_string();
    quote! {
            #ty_ident::#ident => f.write_str(#ident_string),
    }
}
