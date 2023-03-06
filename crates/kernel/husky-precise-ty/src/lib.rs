#![feature(trait_upcasting)]
mod db;
mod entity_path;
mod error;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;

use husky_entity_path::*;
use husky_precise_term::*;
use husky_ty_expectation::*;
use husky_vfs::*;

#[salsa::jar(db = PreciseTypeDb)]
pub struct PreciseTypeJar(
    ty_ontology_path_precise_ty,
    ty_constructor_path_precise_ty,
    form_path_precise_ty,
    trai_path_precise_ty,
);

pub trait HasPreciseType<Db: ?Sized + PreciseTypeDb>: Copy {
    fn precise_ty(self, db: &Db) -> PreciseTerm;
}

impl<Db: ?Sized + PreciseTypeDb> HasPreciseType<Db> for PreciseTerm {
    fn precise_ty(self, db: &Db) -> PreciseTerm {
        match self {
            PreciseTerm::Literal(_) => todo!(),
            PreciseTerm::Symbol(_) => todo!(),
            PreciseTerm::EntityPath(_) => todo!(),
            PreciseTerm::Category(_) => todo!(),
            PreciseTerm::Universe(_) => todo!(),
            PreciseTerm::Curry(_) => todo!(),
            PreciseTerm::Ritchie(_) => todo!(),
            PreciseTerm::Abstraction(_) => todo!(),
            PreciseTerm::Application(_) => todo!(),
            PreciseTerm::Subentity(_) => todo!(),
            PreciseTerm::AsTraitSubentity(_) => todo!(),
            PreciseTerm::TraitConstraint(_) => todo!(),
        }
    }
}
