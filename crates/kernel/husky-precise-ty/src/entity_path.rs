use husky_raw_ty::ty_path_raw_ty;

use crate::*;

#[salsa::tracked(jar = PreciseTypeJar)]
pub fn ty_path_precise_ty(
    db: &dyn PreciseTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> PreciseTypeResult<PreciseTerm> {
    PreciseTerm::from_raw(
        db,
        ty_path_raw_ty(db, path, disambiguation)?,
        match disambiguation {
            TypePathDisambiguation::TypeItselfOrTemplate => todo!(),
            TypePathDisambiguation::InstanceOrConstructor => todo!(),
        },
    )
    .map_err(Into::into)
}
