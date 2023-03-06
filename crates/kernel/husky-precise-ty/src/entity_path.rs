use husky_raw_ty::{
    form_path_raw_ty, trai_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty,
};

use crate::*;
#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_ontology_path_ty(db: &dyn PreciseTermDb, path: TypePath) -> PreciseTypeResult<RawTerm> {
    RawTerm::from_raw(
        db,
        ty_ontology_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
    .map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_constructor_path_ty(
    db: &dyn PreciseTermDb,
    path: TypePath,
) -> PreciseTypeResult<RawTerm> {
    RawTerm::from_raw(
        db,
        ty_constructor_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path),
    )
    .map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn form_path_ty(db: &dyn PreciseTermDb, path: FormPath) -> PreciseTypeResult<RawTerm> {
    RawTerm::from_raw(db, form_path_raw_ty(db, path)?, TermTypeExpectation::Any).map_err(Into::into)
}

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn trai_path_ty(db: &dyn PreciseTermDb, path: TraitPath) -> PreciseTypeResult<RawTerm> {
    RawTerm::from_raw(db, trai_path_raw_ty(db, path)?, TermTypeExpectation::Any).map_err(Into::into)
}
