use crate::*;

pub(crate) fn field_raw_ty(
    db: &dyn RawTypeDb,
    owner_raw_ty: ReducedRawTerm,
    ident: Identifier,
) -> RawTypeResult<Option<ReducedRawTerm>> {
    match owner_raw_ty.raw_term() {
        RawTerm::Literal(_) => unreachable!(),
        RawTerm::Symbol(_) => Ok(None),
        RawTerm::EntityPath(path) => {
            entity_raw_ty_field_raw_ty(db, path.ty_path().expect("should be raw_type"), ident)
        }
        RawTerm::Category(_) => Ok(None),
        RawTerm::Universe(_) => unreachable!(),
        RawTerm::Curry(_) => Ok(None),
        RawTerm::Ritchie(_) => Ok(None),
        RawTerm::Abstraction(_) => unreachable!(),
        RawTerm::Application(raw_ty) => application_raw_ty_field_raw_ty(db, raw_ty, ident),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => unreachable!(),
    }
}

#[salsa::tracked(jar = RawTypeJar,  )]
pub(crate) fn entity_raw_ty_field_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty_path: TypePath,
    ident: Identifier,
) -> RawTypeResult<Option<ReducedRawTerm>> {
    let decl = match db.ty_decl(raw_ty_path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    Err(OriginalRawTypeError::Todo.into())
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn application_raw_ty_field_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty: RawTermApplication,
    ident: Identifier,
) -> RawTypeResult<Option<ReducedRawTerm>> {
    use salsa::DebugWithDb;
    let application_expansion = application_expansion_salsa(db, raw_ty);
    let f = application_expansion.f();
    match f {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::EntityPath(path) => entity_application_raw_ty_field_raw_ty(
            db,
            path.ty_path().expect("should be raw_type"),
            application_expansion.opt_arguments(db).unwrap(),
            ident,
        ),
        RawTerm::Category(_) => todo!(),
        RawTerm::Universe(_) => todo!(),
        RawTerm::Curry(_) => todo!(),
        RawTerm::Ritchie(_) => todo!(),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::Application(_) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
    }
}

fn entity_application_raw_ty_field_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    arguments: &[RawTerm],
    ident: Identifier,
) -> RawTypeResult<Option<ReducedRawTerm>> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    todo!()
}
