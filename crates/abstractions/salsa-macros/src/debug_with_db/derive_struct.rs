use super::*;
use syn::Fields;

pub(super) fn struct_debug_with_db_impl(item: &mut ItemStruct) -> proc_macro2::TokenStream {
    let ident = &item.ident;

    let body = match item.fields {
        syn::Fields::Named(_) => props_struct_ebug_with_db(&item.ident, &mut item.fields),
        syn::Fields::Unnamed(_) => tuple_struct_debug_with_db(&item.ident, &item.fields),
        syn::Fields::Unit => unit_struct_debug_with_db(&item.ident),
    };
    // todo: refactor this as a function
    let generics = &item.generics;
    let generics_without_trais = generics_with_debug_with_db(generics);
    let self_ty = self_ty(ident, generics);
    let where_clause = &item.generics.where_clause;
    quote! {
        impl #generics_without_trais ::salsa::DebugWithDb for #self_ty #where_clause {
            fn debug_fmt_with_db(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &::salsa::Db,) -> ::std::fmt::Result {
                use ::salsa::fmt::{WithFmtContext, WithFmtContextTest};
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                WithFmtContextTest(self).with_fmt_context(|| { #body }, _db)
            }
        }
    }
}

fn props_struct_ebug_with_db(ident: &Ident, fields: &mut Fields) -> proc_macro2::TokenStream {
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = fields
        .iter_mut()
        .filter_map(|field| -> Option<proc_macro2::TokenStream> {
            if has_skip_fmt_attr(field) {
                return None;
            }
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
                        _db
                    )
                );
            };

            Some(quote! {
                #field_debug
            })
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        let mut debug_struct = &mut f.debug_struct(#ident_string);

        #fields

        debug_struct.finish()
    }
}

/// if has #[skip_fmt] attribute, takes it away
fn has_skip_fmt_attr(field: &mut syn::Field) -> bool {
    let has_skip_fmt_attr = field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("skip_fmt"));
    if has_skip_fmt_attr {
        field.attrs = std::mem::take(&mut field.attrs)
            .into_iter()
            .filter(|attr| !attr.path().is_ident("skip_fmt"))
            .collect();
        true
    } else {
        false
    }
}

fn tuple_struct_debug_with_db(ident: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
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
        let mut debug_tuple = &mut f.debug_tuple(#ident_string);

        #fields

        debug_tuple.finish()
    }
}

fn unit_struct_debug_with_db(ident: &Ident) -> proc_macro2::TokenStream {
    let ident_string = ident.to_string();
    quote! {
        f.write_str(#ident_string)
    }
}
