use super::*;

pub(crate) fn form_path_ty(db: &dyn TypeDb, path: FormPath) -> TermResult<EtherealTerm> {
    EtherealTerm::from_raw(
        db,
        form_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
