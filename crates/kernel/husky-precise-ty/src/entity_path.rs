use crate::*;

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> PreciseTypeResult<PreciseTerm> {
    todo!()
    // PreciseTerm::from_precise(db, ty_path_precise_ty(db, path, disambiguation))
}
