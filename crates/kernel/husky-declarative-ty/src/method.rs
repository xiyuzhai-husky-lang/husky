use crate::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_path_ty_method_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
    _ident: Ident,
) -> DeclarativeTypeResult<Option<DeclarativeTerm>> {
    let _signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    todo!()
}
