#![feature(trait_upcasting)]
mod db;
mod entity;
mod error;
mod term;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;

use entity::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_signature::*;
use husky_term::*;
use term::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar(
    ty_entity_ty,
    trai_entity_ty,
    form_entity_ty,
    application_expansion_salsa,
    ApplicationArguments,
);
