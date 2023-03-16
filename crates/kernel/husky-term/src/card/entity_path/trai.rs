use super::*;

pub(crate) fn trai_path_ty(db: &dyn TermDb, path: TraitPath) -> TermResult<Term> {
    trai_path_ty_unchecked(db, path)?.checked(db)
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn trai_path_ty_unchecked(db: &dyn TermDb, path: TraitPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        trai_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
