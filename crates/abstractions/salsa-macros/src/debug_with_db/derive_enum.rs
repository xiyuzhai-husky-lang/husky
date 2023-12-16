use syn::{ItemEnum, Type, Variant};

use super::*;

pub(super) fn enum_debug_with_db_impl(item: &ItemEnum) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let generics = &item.generics;
    let generics_without_trais = generics_with_debug_with_db(generics);
    let self_ty = if item.generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            item.generics.params.iter().map(|param| match param {
                syn::GenericParam::Type(param) => {
                    let ident = param.ident.to_string();
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
    };
    let where_clause = &item.generics.where_clause;
    if item.variants.is_empty() {
        quote! {
        impl #generics_without_trais ::salsa::DebugWithDb for #self_ty #where_clause {
            fn debug_with_db_fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &::salsa::Db,) -> ::std::fmt::Result {
                unreachable!()
            }
        }}
    } else {
        let variants = item
            .variants
            .iter()
            .map(|variant| -> proc_macro2::TokenStream {
                match variant.fields {
                    syn::Fields::Named(_) => enum_struct_variant_debug_with_db(ident, variant),
                    syn::Fields::Unnamed(_) => enum_tuple_variant_debug_with_db(ident, variant),
                    syn::Fields::Unit => enum_unit_variant_debug_with_db(ident, variant),
                }
            })
            .collect::<proc_macro2::TokenStream>();
        quote! {
            impl #generics_without_trais ::salsa::DebugWithDb for #self_ty #where_clause {
                fn debug_with_db_fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &::salsa::Db,) -> ::std::fmt::Result {
                    #[allow(unused_imports)]
                    use ::salsa::debug::helper::Fallback;
                    match self {
                        #variants
                    }
                }
            }
        }
    }
}

fn enum_struct_variant_debug_with_db(
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let variant_string = format!("{}::{}", ty_ident, variant_ident);
    let field_decls = variant
        .fields
        .iter()
        .map(|field| -> proc_macro2::TokenStream {
            let field_ident = field.ident.as_ref().unwrap();
            quote_spanned! {
                field_ident.span()=>
                ref #field_ident
            }
        });
    let field_debugs = variant
        .fields
        .iter()
        .map(|field| -> proc_macro2::TokenStream {
            let mut field_ident = field.ident.as_ref().unwrap().clone();
            field_ident.set_span(Span::call_site());
            let field_ident_string = field_ident.to_string();
            let field_ty = &field.ty;
            // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
            let field_debug = quote! {
                debug_struct = debug_struct.field(
                    #field_ident_string,
                    &::salsa::debug::helper::SalsaDebug::<#field_ty>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        #field_ident,
                        _db
                    )
                );
            };

            quote! {
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
            #ty_ident::#variant_ident { #(#field_decls),* }  => {
                let mut debug_struct = &mut f.debug_struct(#variant_string);

                #field_debugs

                debug_struct.finish()
            }
    }
}

fn enum_tuple_variant_debug_with_db(
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let variant_string = format!("{}::{}", ty_ident, ident);
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let name_field = |field_idx| format_ident!("v{}", field_idx);
    let field_decls =
        variant
            .fields
            .iter()
            .enumerate()
            .map(|(field_idx, _field)| -> proc_macro2::TokenStream {
                let field_ident = name_field(field_idx);
                quote! {
                    ref #field_ident
                }
            });
    let field_debugs = variant
        .fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
            let field_ident = name_field(field_idx);
            let field_ty = &field.ty;

            let field_debug = quote! {
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<#field_ty>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &#field_ident,
                        _db
                    )
                );
            };

            quote! {
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
            #ty_ident::#ident(#(#field_decls),*) => {
                let mut debug_tuple = &mut f.debug_tuple(#variant_string);

                #field_debugs

                debug_tuple.finish()
            }
    }
}

fn enum_unit_variant_debug_with_db(
    ty_ident: &Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let variant_string = format!("{}::{}", ty_ident, ident);
    quote! {
            #ty_ident::#ident => f.write_str(#variant_string),
    }
}
