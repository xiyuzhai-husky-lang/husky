use crate::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_path_ty_method_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    _ident: Ident,
) -> DeclarativeTypeResult<Option<DeclarativeTerm>> {
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedDeclarativeTypeError::TypePathMethodFnDeclError.into()),
    };
    let _signature = match db.ty_declarative_signature_template(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    todo!()
}
