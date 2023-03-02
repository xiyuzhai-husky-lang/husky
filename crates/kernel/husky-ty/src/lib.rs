#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
mod db;
mod entity;
mod error;
mod expectation;
mod field;
mod intrinsic_ty;
mod method;
mod term;
#[cfg(test)]
mod tests;
mod ty_call;
mod variance;

pub use self::db::*;
pub use self::entity::entity_path_ty;
pub use self::entity::*;
pub use self::error::*;
pub use self::expectation::*;
pub use self::intrinsic_ty::*;
pub use self::term::*;

use field::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_signature::*;
use husky_term::*;
use husky_vfs::Toolchain;
use husky_word::*;
use method::*;
#[cfg(test)]
use tests::*;
use ty_call::*;
use variance::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar(
    ty_path_ty,
    trai_entity_ty,
    form_entity_ty,
    ty_entity_variances,
    ty_entity_variance_reprs,
    ty_entity_variance_crate_dependencies,
    trai_entity_variances,
    trai_entity_variance_reprs,
    trai_entity_variance_crate_dependencies,
    form_entity_variances,
    form_entity_variance_reprs,
    form_entity_variance_crate_dependencies,
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
