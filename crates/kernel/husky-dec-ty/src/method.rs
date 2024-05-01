use crate::*;
use husky_dec_signature::signature::HasDecTemplate;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_path_ty_method_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
    _ident: Ident,
) -> DeclarativeTypeResult<Option<DecTerm>> {
    let _signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    todo!()
}
