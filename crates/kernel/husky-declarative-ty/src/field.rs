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
    let signature = match db.ty_declarative_signature_template_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    Ok(match signature {
        TypeDeclarativeSignatureTemplate::RegularStruct(signature) => signature
            .fields(db)
            .iter()
            .find_map(|field| (field.ident() == ident).then_some(field.ty())),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Enum(_)
        | TypeDeclarativeSignatureTemplate::UnitStruct(_)
        | TypeDeclarativeSignatureTemplate::TupleStruct(_)
        | TypeDeclarativeSignatureTemplate::Inductive(_)
        | TypeDeclarativeSignatureTemplate::Extern(_)
        | TypeDeclarativeSignatureTemplate::Union(_) => None,
    })
}
