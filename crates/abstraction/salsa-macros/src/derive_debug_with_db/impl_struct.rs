use super::*;
use syn::Fields;

pub(super) fn struct_debug_with_db_impl(
    db_path: &Path,
    item: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let _ident_string = ident.to_string();

    let body = match item.fields {
        syn::Fields::Named(_) => struct_regular_fields_debug_with_db(&item.ident, &item.fields),
        syn::Fields::Unnamed(_) => struct_tuple_fields_debug_with_db(&item.ident, &item.fields),
        syn::Fields::Unit => todo!("unit struct debug with db"),
    };
    let generic_decls = if item.generics.params.is_empty() {
        quote! { _Db:  #db_path + ?Sized }
    } else {
        let item_generics_punctuated = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            item.generics.params.iter().map(|param| match param {
                syn::GenericParam::Type(param) => {
                    if param.bounds.is_empty() {
                        quote! { #param: ::salsa::DebugWithDb<_Db> }
                    } else {
                        quote! { #param + ::salsa::DebugWithDb<_Db> }
                    }
                }
                syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => quote! { #param },
            }),
        );
        quote! { _Db:  #db_path + ?Sized, #item_generics_punctuated }
    };
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
        impl<#generic_decls> ::salsa::DebugWithDb<_Db> for #self_ty #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &_Db, _level: salsa::DebugFormatLevel) -> ::std::fmt::Result {
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                #body
            }
        }
    }
}

fn struct_regular_fields_debug_with_db(ident: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = fields
        .iter()
        .enumerate()
        .map(|(_field_idx, field)| -> proc_macro2::TokenStream {
            let field_ident = field.ident.as_ref().unwrap();
            let field_ident_string = field_ident.to_string();
            let field_ty = &field.ty;

            let field_debug = quote! {
                debug_struct = debug_struct.field(
                    #field_ident_string,
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.#field_ident,
                        _db,
                        _level.next()
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

fn struct_tuple_fields_debug_with_db(ident: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
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
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.#field_idx,
                        _db,
                        _level.next()
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
