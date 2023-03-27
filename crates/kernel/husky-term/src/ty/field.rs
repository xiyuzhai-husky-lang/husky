use super::*;
use husky_raw_ty::ty_path_field_raw_ty;

pub(crate) fn field_ty(db: &dyn TermDb, owner_ty: Term, ident: Ident) -> TermResult<Option<Term>> {
    match owner_ty {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Variable(_) => todo!(),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => ty_ontology_path_field_ty(db, path, ident),
            TermEntityPath::TypeConstructor(_) => {
                p!(owner_ty.debug(db), ident.debug(db));
                todo!()
            }
        },
        Term::Category(_) => todo!(),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => {
            p!(owner_ty.debug(db));
            todo!()
        }
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

fn ty_ontology_path_field_ty(
    db: &dyn TermDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<Term>> {
    let Some(field_raw_ty) = ty_path_field_raw_ty(db, path, ident)? else {
        return Ok(None)
    };
    Ok(Some(Term::from_raw(
        db,
        field_raw_ty,
        TermTypeExpectation::FinalDestinationEqsSort,
    )?))
}
