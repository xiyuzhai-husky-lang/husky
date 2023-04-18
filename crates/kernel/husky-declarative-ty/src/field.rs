use crate::*;
use husky_decl::HasDecl;

#[salsa::tracked(jar = DeclarativeTypeJar,  )]
pub fn ty_path_field_raw_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    ident: Ident,
) -> DeclarativeTypeResult<Option<DeclarativeTerm>> {
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedDeclarativeTypeError::TypePathFieldDeclError.into()),
    };
    let signature = match db.ty_declarative_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    Ok(match signature {
        TypeDeclarativeSignature::RegularStruct(signature) => signature
            .fields(db)
            .iter()
            .find_map(|field| (field.ident() == ident).then_some(field.ty())),
        TypeDeclarativeSignature::Record(_) => todo!(),
        TypeDeclarativeSignature::Structure(_) => todo!(),
        TypeDeclarativeSignature::Enum(_)
        | TypeDeclarativeSignature::UnitStruct(_)
        | TypeDeclarativeSignature::TupleStruct(_)
        | TypeDeclarativeSignature::Inductive(_)
        | TypeDeclarativeSignature::Extern(_)
        | TypeDeclarativeSignature::Union(_) => None,
    })
}
