use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_method_fn_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
) -> TypeMethodFnEtherealSignatureTemplate {
    todo!()
}
