use super::*;
use syn::Fields;

pub(super) fn struct_debug_with_db_impl(
    db_path: &Path,
    item: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let ident_string = ident.to_string();

    let body = match item.fields {
        syn::Fields::Named(_) => struct_regular_fields_debug_with_db(&item.ident, &item.fields),
        syn::Fields::Unnamed(_) => struct_tuple_fields_debug_with_db(&item.ident, &item.fields),
        syn::Fields::Unit => todo!("unit struct debug with db"),
    };

    quote! {
        impl<_Db:  #db_path + ?Sized> ::salsa::DebugWithDb<_Db> for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &_Db, _include_all_fields: bool) -> ::std::fmt::Result {
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
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
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
                        _include_all_fields
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
