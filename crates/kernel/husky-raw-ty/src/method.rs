use crate::*;

#[salsa::tracked(jar = RawTypeJar)]
pub fn ty_path_ty_method_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty_path: TypePath,
    _ident: Ident,
) -> RawTypeResult<Option<RawTerm>> {
    let decl = match db.ty_decl(raw_ty_path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypePathMethodDeclError.into()),
    };
    let _signature = match db.ty_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    todo!()
}
