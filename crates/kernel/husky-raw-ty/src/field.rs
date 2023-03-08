use crate::*;

pub(crate) fn field_raw_ty(
    db: &dyn RawTypeDb,
    owner_raw_ty: RawTerm,
    ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    match owner_raw_ty {
        RawTerm::Literal(_) => unreachable!(),
        RawTerm::Symbol(_) => Ok(None),
        RawTerm::EntityPath(path) => {
            ty_path_field_raw_ty(db, path.ty_path().expect("should be raw_type"), ident)
        }
        RawTerm::Category(_) => Ok(None),
        RawTerm::Universe(_) => unreachable!(),
        RawTerm::Curry(_) => Ok(None),
        RawTerm::Ritchie(_) => Ok(None),
        RawTerm::Abstraction(_) => unreachable!(),
        RawTerm::ExplicitApplication(raw_ty) => application_raw_ty_field_raw_ty(db, raw_ty, ident),
        RawTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => unreachable!(),
        RawTerm::LeashOrBitNot(_) => todo!(),
        RawTerm::List(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar,  )]
pub(crate) fn ty_path_field_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty_path: TypePath,
    _ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let decl = match db.ty_decl(raw_ty_path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypePathFieldDeclError.into()),
    };
    let _signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    Err(OriginalRawTypeError::Todo.into())
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn application_raw_ty_field_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty: RawTermExplicitApplication,
    ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let application_expansion = application_expansion_salsa(db, raw_ty);
    let f = application_expansion.f();
    match f {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::EntityPath(path) => ty_path_application_raw_ty_field_raw_ty(
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
        RawTerm::ExplicitApplication(_) => todo!(),
        RawTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
        RawTerm::LeashOrBitNot(_) => todo!(),
        RawTerm::List(_) => todo!(),
    }
}

fn ty_path_application_raw_ty_field_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    _arguments: &[RawTerm],
    _ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypePathApplicationFieldDeclError.into()),
    };
    let _signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    todo!()
}
