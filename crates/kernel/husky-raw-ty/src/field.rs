use crate::*;
use husky_decl::HasDecl;

#[salsa::tracked(jar = RawTypeJar,  )]
pub fn ty_path_field_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypePathFieldDeclError.into()),
    };
    let signature = match db.ty_signature_from_decl(decl) {
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
