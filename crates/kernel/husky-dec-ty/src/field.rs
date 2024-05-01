use crate::*;
use husky_dec_signature::signature::{major_item::ty::TypeDecTemplate, HasDecTemplate};

#[salsa::tracked(jar = DeclarativeTypeJar,  )]
pub fn ty_path_field_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
    ident: Ident,
) -> DeclarativeTypeResult<Option<DecTerm>> {
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    Ok(match signature {
        TypeDecTemplate::PropsStruct(signature) => signature
            .fields(db)
            .iter()
            .find_map(|field| (field.ident() == ident).then_some(field.ty())),
        TypeDecTemplate::Structure(_) => todo!(),
        TypeDecTemplate::Enum(_)
        | TypeDecTemplate::UnitStruct(_)
        | TypeDecTemplate::TupleStruct(_)
        | TypeDecTemplate::Inductive(_)
        | TypeDecTemplate::Extern(_)
        | TypeDecTemplate::Union(_) => None,
    })
}
