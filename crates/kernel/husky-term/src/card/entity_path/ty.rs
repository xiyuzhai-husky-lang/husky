use super::*;

pub fn ty_path_ty(
    db: &dyn TermDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> TermResult<Term> {
    match disambiguation {
        TypePathDisambiguation::Ontology => ty_ontology_path_ty_unchecked(db, path),
        TypePathDisambiguation::Constructor => ty_constructor_path_ty_unchecked(db, path),
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn ty_ontology_path_ty_unchecked(db: &dyn TermDb, path: TypePath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        ty_ontology_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn ty_constructor_path_ty_unchecked(
    db: &dyn TermDb,
    path: TypePath,
) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        ty_constructor_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
