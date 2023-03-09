use crate::*;
use husky_raw_ty::ty_path_ty_method_raw_ty;

pub(crate) fn ty_method_ty(
    db: &dyn TermDb,
    owner_ty: Term,
    ident: Ident,
) -> TermResult<Option<Term>> {
    match owner_ty {
        Term::Literal(_) => unreachable!(),
        Term::Symbol(_) => Ok(None),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => ty_ontology_path_ty_method_ty(db, path, ident),
            TermEntityPath::TypeConstructor(_) => todo!(),
        },
        Term::Category(_) => Ok(None),
        Term::Universe(_) => unreachable!(),
        Term::Curry(_) => Ok(None),
        Term::Ritchie(_) => Ok(None),
        Term::Abstraction(_) => unreachable!(),
        Term::Application(raw_ty) => term_application_ty_method_ty(db, raw_ty, ident),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => unreachable!(),
    }
}

pub(crate) fn ty_ontology_path_ty_method_ty(
    db: &dyn TermDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<Term>> {
    let Some(method_raw_ty) = ty_path_ty_method_raw_ty(db, path, ident)? else {
        return Ok(None)
    };
    Ok(Some(Term::from_raw(
        db,
        method_raw_ty,
        TermTypeExpectation::FinalDestinationEqsSort,
    )?))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_ty_method_ty(
    db: &dyn TermDb,
    raw_ty: TermApplication,
    ident: Ident,
) -> TermResult<Option<Term>> {
    todo!()
    // let application_expansion = application_expansion_salsa(db, raw_ty);
    // let f = application_expansion.f();
    // match f {
    //     Term::Literal(_) => todo!(),
    //     Term::Symbol(_) => todo!(),
    //     Term::EntityPath(path) => ty_path_application_raw_ty_method_raw_ty(
    //         db,
    //         path.ty_path().expect("should be raw_type"),
    //         application_expansion.opt_arguments(db).unwrap(),
    //         ident,
    //     ),
    //     Term::Category(_) => todo!(),
    //     Term::Universe(_) => todo!(),
    //     Term::Curry(_) => todo!(),
    //     Term::Ritchie(_) => todo!(),
    //     Term::Abstraction(_) => todo!(),
    //     Term::ExplicitApplication(_) => todo!(),
    //     Term::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
    //     Term::Subentity(_) => todo!(),
    //     Term::AsTraitSubentity(_) => todo!(),
    //     Term::TraitConstraint(_) => todo!(),
    //     Term::LeashOrBitNot(_) => todo!(),
    //     Term::List(_) => todo!(),
    // }
}

fn ty_ontology_path_application_ty_method_ty(
    db: &dyn TermDb,
    path: TypePath,
    _arguments: &[Term],
    _ident: Ident,
) -> TermResult<Option<Term>> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(TermError::TypePathApplicationMethodDeclError),
    };
    let _signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(TermError::SignatureError),
    };
    todo!()
}
