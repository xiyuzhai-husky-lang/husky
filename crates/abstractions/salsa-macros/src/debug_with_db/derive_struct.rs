use super::*;
use syn::{Fields, Type};

pub(super) fn struct_debug_with_db_impl(
    db_trai: &Path,
    jar_ty: &Type,
    item: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &item.ident;

    let body = match item.fields {
        syn::Fields::Named(_) => {
            struct_regular_fields_debug_with_db(db_trai, jar_ty, &item.ident, &item.fields)
        }
        syn::Fields::Unnamed(_) => {
            struct_tuple_fields_debug_with_db(db_trai, jar_ty, &item.ident, &item.fields)
        }
        syn::Fields::Unit => todo!("unit struct debug with db"),
    };
    // todo: refactor this as a function
    let generics = &item.generics;
    let generics_without_db = generics_without_db(generics, db_trai);
    let self_ty = if item.generics.params.is_empty() {
        quote! { #ident }
    } else {
        let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            item.generics.params.iter().map(|param| match param {
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
    };
    let where_clause = &item.generics.where_clause;
    quote! {
        impl #generics_without_db ::salsa::DebugWithDb for #self_ty #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &::salsa::Db) -> ::std::fmt::Result {
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                #body
            }
        }
    }
}

fn struct_regular_fields_debug_with_db(
    db_trai: &Path,
    jar_ty: &Type,
    ident: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = fields
        .iter()
        .enumerate()
        .map(|(_field_idx, field)| -> proc_macro2::TokenStream {
            let mut field_ident = field.ident.as_ref().unwrap().clone();
            field_ident.set_span(Span::mixed_site());
            let field_ident_string = field_ident.to_string();
            let field_ty = &field.ty;

            let field_debug = quote! {
                debug_struct = debug_struct.field(
                    #field_ident_string,
                    &::salsa::debug::helper::SalsaDebug::<#field_ty>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.#field_ident,
                        _db.as_jar_db_dyn::<#jar_ty>()
                    )
                );
            };

            quote! {
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        let mut debug_struct = &mut f.debug_struct(#ident_string);

        #fields

        debug_struct.finish()
    }
}

fn struct_tuple_fields_debug_with_db(
    db_trai: &Path,
    jar_ty: &Type,
    ident: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
            let field_idx = syn::Index {
                index: field_idx as u32,
                span: field.span(),
            };
            let field_ty = &field.ty;

            let field_debug = quote! {
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<#field_ty>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.#field_idx,
                        _db.as_jar_db_dyn::<#jar_ty>()
                    )
                );
            };

            quote! {
                #field_debug
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        let mut debug_tuple = &mut f.debug_tuple(#ident_string);

        #fields

        debug_tuple.finish()
    }
}
