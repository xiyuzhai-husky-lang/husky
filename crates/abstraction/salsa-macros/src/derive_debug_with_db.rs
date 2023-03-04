mod impl_enum;
mod impl_struct;

use crate::options::Options;
use impl_enum::*;
use impl_struct::*;
use proc_macro2::Literal;
use syn::{punctuated::Punctuated, Type};
use syn::{spanned::Spanned, Item};
use syn::{Field, FieldsUnnamed, Ident, ItemStruct, Path, Token};

// Source:
//
// #[salsa::jar(db = Jar0Db)]
// pub struct Jar0(Entity0, Ty0, EntityComponent0, my_func);

pub(crate) fn derive_debug_with_db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let options = syn::parse_macro_input!(args as Args);
    let db_path = match options.db_trai {
        Some(db_path) => db_path,
        None => panic!("no `db` specified"),
    };
    let item = syn::parse_macro_input!(input as Item);
    let impl_debug_with_db = match item {
        Item::Enum(ref item) => enum_debug_with_db_impl(&db_path, item),
        Item::Struct(ref item) => struct_debug_with_db_impl(&db_path, item),
        _ => panic!("expect enum or struct for `derive_debug_with_db`"),
    };
    quote! {
        #item

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

    const JAR: bool = true;

    const DATA: bool = false;

    const DB: bool = true;

    const RECOVERY_FN: bool = false;

    const LRU: bool = false;

    const CONSTRUCTOR: bool = false;

    const OVERRIDE_DEBUG: bool = false;
}
