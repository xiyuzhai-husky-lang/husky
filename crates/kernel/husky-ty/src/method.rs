use husky_print_utils::p;

use crate::*;

pub(crate) fn ty_method_ty(
    db: &dyn TermDb,
    owner_ty: Term,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    match owner_ty {
        Term::Literal(_) => unreachable!(),
        Term::Symbol(_) => Ok(None),
        Term::EntityPath(path) => {
            entity_ty_method_ty(db, path.ty_ontology().expect("should be type"), ident)
        }
        Term::Category(_) => Ok(None),
        Term::Universe(_) => unreachable!(),
        Term::Curry(_) => Ok(None),
        Term::Ritchie(_) => Ok(None),
        Term::Abstraction(_) => unreachable!(),
        Term::Application(ty) => application_ty_method_ty(db, ty, ident),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => unreachable!(),
    }
}

#[salsa::tracked(jar = TermJar,  )]
pub(crate) fn entity_ty_method_ty(
    db: &dyn TermDb,
    ty_path: TypePath,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    todo!()
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn application_ty_method_ty(
    db: &dyn TermDb,
    ty: TermApplication,
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    use salsa::DebugWithDb;
    let application_expansion = application_expansion_salsa(db, ty);
    let f = application_expansion.f();
    match f {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::EntityPath(path) => entity_application_ty_method_ty(
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

fn entity_application_ty_method_ty(
    db: &dyn TermDb,
    path: TypePath,
    arguments: &[Term],
    ident: Identifier,
) -> TypeResult<Option<Term>> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(TermError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(TermError::SignatureError.into()),
    };
    todo!()
}
