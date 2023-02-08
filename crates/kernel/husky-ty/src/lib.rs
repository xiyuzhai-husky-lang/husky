#![feature(trait_upcasting)]
mod db;
mod entity;
mod error;
mod method;
mod term;
#[cfg(test)]
mod tests;
mod variance;

pub use db::*;
pub use error::*;
pub use term::*;

use entity::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_signature::*;
use husky_term::*;
use husky_word::*;
use method::*;
use variance::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar(
    ty_entity_ty,
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
);
