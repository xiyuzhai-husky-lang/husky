use proc_macro2::Literal;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Field, FieldsUnnamed, Ident, ItemStruct, Path, Token};

use crate::options::Options;

// Source:
//
// #[salsa::jar(db = Jar0Db)]
// pub struct Jar0(Entity0, Ty0, EntityComponent0, my_func);

pub(crate) fn derive_debug_with_db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let options = syn::parse_macro_input!(args as Args);
    let db_path = match options.db_path {
        Some(v) => v,
        None => panic!("no `db` specified"),
    };
    let input = syn::parse_macro_input!(input as ItemStruct);
    let impl_debug_with_db = impl_struct_debug_with_db(&db_path, &input);
    quote! {
        #input

        #impl_debug_with_db
    }
    .into()
}

type Args = Options<DeriveDebugWithDb>;

struct DeriveDebugWithDb;

impl crate::options::AllowedOptions for DeriveDebugWithDb {
    const RETURN_REF: bool = false;

    const SPECIFY: bool = false;

    const NO_EQ: bool = false;

    const SINGLETON: bool = false;

    const JAR: bool = false;

    const DATA: bool = false;

    const DB: bool = true;

    const RECOVERY_FN: bool = false;

    const LRU: bool = false;

    const CONSTRUCTOR_NAME: bool = false;

    const OVERRIDE_DEBUG: bool = false;
}

pub(crate) fn impl_struct_debug_with_db(
    db_path: &Path,
    input: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let ident_string = ident.to_string();

    match input.fields {
        syn::Fields::Named(_) => impl_regular_struct_debug_with_db(db_path, input),
        syn::Fields::Unnamed(_) => impl_tuple_struct_debug_with_db(db_path, input),
        syn::Fields::Unit => todo!("unit struct debug with db"),
    }
}

pub(crate) fn impl_regular_struct_debug_with_db(
    db_path: &Path,
    input: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = input
        .fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
            let field_ident = field.ident.as_ref().unwrap();
            let field_ident_string = field_ident.to_string();
            let field_ty = &field.ty;

            let field_debug = quote_spanned! { field.span() =>
                debug_struct = debug_struct.field(
                    #field_ident_string,
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, DB>::salsa_debug(
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
        impl<DB: #db_path + ?Sized> ::salsa::DebugWithDb<DB> for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &DB, _include_all_fields: bool) -> ::std::fmt::Result {
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                let mut debug_struct = &mut f.debug_struct(#ident_string);

                #fields

                debug_struct.finish()
            }
        }
    }
}

pub(crate) fn impl_tuple_struct_debug_with_db(
    db_path: &Path,
    input: &ItemStruct,
) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let ident_string = ident.to_string();
    // `::salsa::debug::helper::SalsaDebug` will use `DebugWithDb` or fallbak to `Debug`
    let fields = input
        .fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| -> proc_macro2::TokenStream {
            let field_idx = syn::Index {
                index: field_idx as u32,
                span: field.span(),
            };
            let field_ty = &field.ty;

            let field_debug = quote_spanned! { field.span() =>
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<#field_ty, DB>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.#field_idx,
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
        impl<DB: #db_path + ?Sized> ::salsa::DebugWithDb<DB> for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, _db: &DB, _include_all_fields: bool) -> ::std::fmt::Result {
                #[allow(unused_imports)]
                use ::salsa::debug::helper::Fallback;
                let mut debug_tuple = &mut f.debug_tuple(#ident_string);

                #fields

                debug_tuple.finish()
            }
        }
    }
}
