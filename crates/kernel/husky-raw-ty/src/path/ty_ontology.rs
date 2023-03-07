use super::*;

#[salsa::tracked(jar = RawTypeJar)]
pub fn ty_constructor_path_raw_ty(db: &dyn RawTypeDb, path: TypePath) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypeConstructorDeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = raw_ty_entity_variances(db, path) else {
        todo!()
    };
    let mut self_ty: RawTerm = path.into();
    for implicit_parameter in signature.implicit_parameters(db) {
        self_ty = RawTermApplication::new(db, self_ty, implicit_parameter.symbol().into()).into()
    }
    match signature {
        TypeSignature::Enum(_) => Err(todo!()),
        TypeSignature::RegularStruct(signature) => {
            regular_struct_ty_constructor_path_raw_ty(db, signature)
        }
        TypeSignature::UnitStruct(_) => todo!(),
        TypeSignature::TupleStruct(_) => todo!(),
        TypeSignature::Record(_) => todo!(),
        TypeSignature::Inductive(_) => todo!(),
        TypeSignature::Structure(_) => todo!(),
        TypeSignature::Foreign(_) => todo!(),
        TypeSignature::Union(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn regular_struct_ty_constructor_path_raw_ty(
    db: &dyn RawTypeDb,
    signature: RegularStructTypeSignature,
) -> RawTypeResult<RawTerm> {
    todo!()
}
