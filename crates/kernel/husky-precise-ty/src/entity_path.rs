use husky_raw_ty::{ty_constructor_raw_ty, ty_ontology_raw_ty};

use crate::*;

pub fn ty_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> PreciseTypeResult<PreciseTerm> {
    match disambiguation {
        TypePathDisambiguation::TypeItselfOrTemplate => PreciseTerm::from_raw(
            db,
            ty_ontology_raw_ty(db, path)?,
            TypeExpectation::FinalDestinationEqsSort,
        ),
        TypePathDisambiguation::InstanceOrConstructor => PreciseTerm::from_raw(
            db,
            ty_constructor_raw_ty(db, path)?,
            TypeExpectation::FinalDestinationEqsNonSortTypePath(path),
        ),
    }
    .map_err(Into::into)
}
#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_path_ontology_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(
        db,
        ty_ontology_raw_ty(db, path)?,
        TypeExpectation::FinalDestinationEqsSort,
    )
    .map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_path_constructor_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(
        db,
        ty_constructor_raw_ty(db, path)?,
        TypeExpectation::FinalDestinationEqsNonSortTypePath(path),
    )
    .map_err(Into::into)
}
