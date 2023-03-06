use crate::*;
use husky_ty::{
    form_path_ty_unchecked, trai_path_ty_unchecked, ty_constructor_path_ty_unchecked,
    ty_ontology_path_ty_unchecked,
};

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_ontology_path_ty_unchecked(
    db: &dyn ValidTermDb,
    path: TypePath,
) -> ValidTypeResult<RawTerm> {
    Ok(RawTerm::from_precise(
        db,
        ty_ontology_path_ty_unchecked(db, path)?,
    )?)
}

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_constructor_path_ty_unchecked(
    db: &dyn ValidTermDb,
    path: TypePath,
) -> ValidTypeResult<RawTerm> {
    Ok(RawTerm::from_precise(
        db,
        ty_constructor_path_ty_unchecked(db, path)?,
    )?)
}

#[salsa::tracked(jar = ValidTypeJar)]
pub fn form_path_ty_unchecked(db: &dyn ValidTermDb, path: FormPath) -> ValidTypeResult<RawTerm> {
    Ok(RawTerm::from_precise(
        db,
        form_path_ty_unchecked(db, path)?,
    )?)
}

#[salsa::tracked(jar = ValidTypeJar)]
pub fn trai_path_ty_unchecked(db: &dyn ValidTermDb, path: TraitPath) -> ValidTypeResult<RawTerm> {
    Ok(RawTerm::from_precise(
        db,
        trai_path_ty_unchecked(db, path)?,
    )?)
}
