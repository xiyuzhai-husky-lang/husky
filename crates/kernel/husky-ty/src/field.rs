use crate::*;

pub(crate) fn field_ty(
    db: &dyn TypeDb,
    owner_ty: Term,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    match owner_ty {
        Term::Literal(_) => unreachable!(),
        Term::Symbol(_) => Ok(None),
        Term::EntityPath(path) => {
            entity_ty_field_ty(db, path.ty_ontology().expect("should be type"), ident)
        }
        Term::Category(_) => Ok(None),
        Term::Universe(_) => unreachable!(),
        Term::Curry(_) => Ok(None),
        Term::Ritchie(_) => Ok(None),
        Term::Abstraction(_) => unreachable!(),
        Term::Application(ty) => application_ty_field_ty(db, ty, ident),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => unreachable!(),
    }
}

#[salsa::tracked(jar = TypeJar,  )]
pub(crate) fn entity_ty_field_ty(
    db: &dyn TypeDb,
    ty_path: TypePath,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn application_ty_field_ty(
    db: &dyn TypeDb,
    ty: TermApplication,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    use salsa::DebugWithDb;
    let application_expansion = application_expansion_salsa(db, ty);
    let f = application_expansion.f();
    match f {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::EntityPath(path) => entity_application_ty_field_ty(
            db,
            path.ty_ontology().expect("should be type"),
            application_expansion.opt_arguments(db).unwrap(),
            ident,
        ),
        Term::Category(_) => todo!(),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => todo!(),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

fn entity_application_ty_field_ty(
    db: &dyn TypeDb,
    path: TypePath,
    arguments: &[Term],
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    todo!()
}
