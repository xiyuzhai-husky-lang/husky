use crate::*;
use husky_decl::HasDecl;

#[salsa::tracked(jar = DeclarativeTypeJar,  )]
pub fn ty_path_field_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    ident: Ident,
) -> DeclarativeTypeResult<Option<DeclarativeTerm>> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    Ok(match signature {
        TypeDeclarativeSignatureTemplate::PropsStruct(signature) => signature
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
