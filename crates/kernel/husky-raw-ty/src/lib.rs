#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
mod db;
mod entity_path;
mod error;
mod field;
mod method;
mod term;
#[cfg(test)]
mod tests;
mod ty_call;
mod variance;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::term::*;

use self::field::*;
use self::method::*;
#[cfg(test)]
use self::tests::*;
use self::ty_call::*;
use self::variance::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_raw_term::*;
use husky_signature::*;
use husky_term_prelude::*;
use husky_ty_expectation::TypePathDisambiguation;
use husky_vfs::Toolchain;
use husky_word::*;

#[salsa::jar(db = RawTypeDb)]
pub struct RawTypeJar(
    ty_ontology_path_raw_ty,
    ty_constructor_path_raw_ty,
    trai_path_raw_ty,
    form_path_raw_ty,
    raw_ty_entity_variances,
    raw_ty_entity_variance_reprs,
    raw_ty_entity_variance_crate_dependencies,
    trai_entity_variances,
    trai_entity_variance_reprs,
    trai_entity_variance_crate_dependencies,
    form_entity_variances,
    form_entity_variance_reprs,
    form_entity_variance_crate_dependencies,
    application_expansion_salsa,
    ApplicationArguments,
    entity_raw_ty_method_raw_ty,
    application_raw_ty_method_raw_ty,
    entity_raw_ty_field_raw_ty,
    application_raw_ty_field_raw_ty,
    application_raw_term_raw_ty,
    ty_path_raw_ty_call_raw_ty,
    RawTermSymbols,
    raw_term_curry_symbols,
    raw_term_ritchie_symbols,
    raw_term_application_symbols,
);
