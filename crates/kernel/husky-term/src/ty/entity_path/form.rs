use super::*;

pub(crate) fn form_path_ty(db: &dyn TermDb, path: FormPath) -> TermResult<Term> {
    form_path_ty_unchecked(db, path)?.checked(db)
}

pub(crate) fn form_path_ty_unchecked(db: &dyn TermDb, path: FormPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        form_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
