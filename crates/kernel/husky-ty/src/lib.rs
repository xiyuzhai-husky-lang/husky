#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
mod db;
mod entity_path;
mod error;
mod field;
mod intrinsic_ty;
mod method;
mod term;
#[cfg(test)]
mod tests;
mod ty_call;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::intrinsic_ty::*;
pub use self::term::*;

use self::field::*;
use self::method::*;
#[cfg(test)]
use self::tests::*;
use self::ty_call::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_signature::*;
use husky_term::*;
use husky_term_attrs::*;
use husky_ty_expectation::*;
use husky_vfs::Toolchain;
use husky_word::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar(
    ty_ontology_path_ty,
    ty_constructor_path_ty,
    trai_path_ty,
    form_path_ty,
    application_expansion_salsa,
    ApplicationArguments,
    entity_ty_method_ty,
    application_ty_method_ty,
    entity_ty_field_ty,
    application_ty_field_ty,
    application_term_ty,
    ty_path_ty_call_ty,
    TermSymbols,
    term_curry_symbols,
    term_ritchie_symbols,
    term_application_symbols,
);
