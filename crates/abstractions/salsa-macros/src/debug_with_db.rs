mod derive_enum;
mod derive_struct;

use self::derive_enum::*;
use self::derive_struct::*;
use crate::options::Options;
use husky_macro_utils::generics_with_debug_with_db;
use proc_macro2::Span;
use syn::{spanned::Spanned, Item};
use syn::{Ident, ItemStruct, Path};

type Args = Options<DeriveDebugWithDb>;

pub(crate) fn debug_with_db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let options = syn::parse_macro_input!(args as Args);
    let item = syn::parse_macro_input!(input as Item);
    let impl_debug_with_db = match item {
        Item::Enum(ref item) => enum_debug_with_db_impl(item),
        Item::Struct(ref item) => struct_debug_with_db_impl(item),
        _ => panic!("expect enum or struct for `derive_debug_with_db`"),
    };
    quote! {
        #item

        #impl_debug_with_db
    }
    .into()
}

struct DeriveDebugWithDb;

impl crate::options::AllowedOptions for DeriveDebugWithDb {
    const RETURN_REF: bool = false;

    const SPECIFY: bool = false;

    const NO_EQ: bool = false;

    const SINGLETON: bool = false;

    const JAR: bool = true;

    const DATA: bool = false;

    const DB: bool = true;

    const RECOVERY_FN: bool = false;

    const LRU: bool = false;

    const CONSTRUCTOR: bool = false;

    const OVERRIDE_DEBUG: bool = false;
}
