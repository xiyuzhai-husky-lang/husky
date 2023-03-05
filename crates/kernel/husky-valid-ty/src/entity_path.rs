use crate::*;

#[salsa::tracked(jar = ValidTypeJar)]
pub fn ty_path_valid_ty(
    db: &dyn ValidTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> ValidTypeResult<ValidTerm> {
    todo!()
    // ValidTerm::from_valid(db, ty_path_valid_ty(db, path, disambiguation))
}
