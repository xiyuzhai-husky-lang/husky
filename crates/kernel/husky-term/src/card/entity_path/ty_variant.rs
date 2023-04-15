use super::*;
use husky_raw_ty::ty_variant_path_raw_ty;

pub(crate) fn ty_variant_path_ty(db: &dyn TermDb, path: TypeVariantPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        ty_variant_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
