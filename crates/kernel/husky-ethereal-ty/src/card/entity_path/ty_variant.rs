use super::*;
use husky_declarative_ty::ty_variant_path_raw_ty;

pub(crate) fn ty_variant_path_ty(
    db: &dyn EtherealTypeDb,
    path: TypeVariantPath,
) -> TermResult<EtherealTerm> {
    EtherealTerm::from_raw(
        db,
        ty_variant_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
