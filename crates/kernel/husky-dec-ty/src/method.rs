use crate::*;
use husky_dec_signature::signature::HasDecTemplate;
use husky_entity_path::path::major_item::ty::TypePath;

#[salsa::tracked(jar = DecTypeJar)]
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
