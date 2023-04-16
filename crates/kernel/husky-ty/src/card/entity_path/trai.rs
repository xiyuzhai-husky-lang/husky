use super::*;

pub(crate) fn trai_path_ty(db: &dyn TypeDb, path: TraitPath) -> TermResult<EtherealTerm> {
    EtherealTerm::from_raw(
        db,
        trai_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
