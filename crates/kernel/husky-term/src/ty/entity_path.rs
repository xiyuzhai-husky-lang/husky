use super::*;
use husky_raw_ty::{
    form_path_raw_ty, trai_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty,
};
use husky_ty_expectation::{TermTypeExpectation, TypePathDisambiguation};

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
        TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path),
    )
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn trai_path_ty_unchecked(db: &dyn TermDb, path: TraitPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        trai_path_raw_ty(db, path)?,
        /* ad hoc but enough */ TermTypeExpectation::Any,
    )
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn form_path_ty_unchecked(db: &dyn TermDb, path: FormPath) -> TermResult<Term> {
    Term::from_raw_unchecked(
        db,
        form_path_raw_ty(db, path)?,
        TermTypeExpectation::FinalDestinationEqsSort,
    )
}
