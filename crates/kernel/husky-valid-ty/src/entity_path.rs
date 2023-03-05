use crate::*;
use husky_precise_ty::ty_path_precise_ty;

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_path_valid_ty(
    db: &dyn ValidTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> ValidTypeResult<ValidTerm> {
    Ok(ValidTerm::from_precise(
        db,
        ty_path_precise_ty(db, path, disambiguation)?,
    ))
}
