use husky_raw_ty::{form_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty};

use crate::*;
#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_ontology_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(
        db,
        ty_ontology_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
    .map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_constructor_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(
        db,
        ty_constructor_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path),
    )
    .map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn form_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: FormPath,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(db, form_path_raw_ty(db, path)?, TermTypeExpectation::Any)
        .map_err(Into::into)
}
