use super::*;

pub(crate) fn ty_path_ty(
    db: &dyn EtherealTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> TermResult<EtherealTerm> {
    match disambiguation {
        TypePathDisambiguation::Ontology => ty_ontology_path_ty_unchecked(db, path),
        TypePathDisambiguation::Constructor => ty_constructor_path_ty_unchecked(db, path),
    }
}

pub(crate) fn ty_ontology_path_ty_unchecked(
    db: &dyn EtherealTypeDb,
    path: TypePath,
) -> TermResult<EtherealTerm> {
    EtherealTerm::from_raw(
        db,
        ty_ontology_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}

pub(crate) fn ty_constructor_path_ty_unchecked(
    db: &dyn EtherealTypeDb,
    path: TypePath,
) -> TermResult<EtherealTerm> {
    EtherealTerm::from_raw(
        db,
        ty_constructor_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
