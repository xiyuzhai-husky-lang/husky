mod ty;
mod ty_item;

pub use self::ty::*;
pub use self::ty_item::*;

use super::*;
use husky_raw_ty::{
    form_path_raw_ty, trai_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty,
};
use husky_ty_expectation::{TermTypeExpectation, TypePathDisambiguation};

#[salsa::tracked(jar = TermJar)]
pub(crate) fn trai_path_ty_unchecked(db: &dyn TermDb, path: TraitPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        trai_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn form_path_ty_unchecked(db: &dyn TermDb, path: FormPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        form_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
