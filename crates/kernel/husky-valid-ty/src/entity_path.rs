use crate::*;
use husky_precise_ty::{
    form_path_precise_ty, ty_constructor_path_precise_ty, ty_ontology_path_precise_ty,
};

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_ontology_path_valid_ty(
    db: &dyn ValidTypeDb,
    path: TypePath,
) -> ValidTypeResult<ValidTerm> {
    Ok(ValidTerm::from_precise(
        db,
        ty_ontology_path_precise_ty(db, path)?,
    )?)
}

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_constructor_path_valid_ty(
    db: &dyn ValidTypeDb,
    path: TypePath,
) -> ValidTypeResult<ValidTerm> {
    Ok(ValidTerm::from_precise(
        db,
        ty_constructor_path_precise_ty(db, path)?,
    )?)
}

#[salsa::tracked(jar = ValidTypeJar)]
pub fn form_path_valid_ty(db: &dyn ValidTypeDb, path: FormPath) -> ValidTypeResult<ValidTerm> {
    Ok(ValidTerm::from_precise(
        db,
        form_path_precise_ty(db, path)?,
    )?)
}
