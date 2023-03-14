use crate::*;

#[salsa::tracked(jar = RawTypeJar,  )]
pub fn ty_path_field_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty_path: TypePath,
    ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let decl = match db.ty_decl(raw_ty_path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypePathFieldDeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    Ok(match signature {
        TypeSignature::RegularStruct(signature) => signature
            .fields(db)
            .iter()
            .find_map(|field| (field.ident() == ident).then_some(field.ty())),
        TypeSignature::Record(_) => todo!(),
        TypeSignature::Structure(_) => todo!(),
        TypeSignature::Enum(_)
        | TypeSignature::UnitStruct(_)
        | TypeSignature::TupleStruct(_)
        | TypeSignature::Inductive(_)
        | TypeSignature::Foreign(_)
        | TypeSignature::Union(_) => None,
    })
}
