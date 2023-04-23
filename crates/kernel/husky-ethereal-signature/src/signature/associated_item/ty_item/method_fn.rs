use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {}

impl HasEtherealSignatureTemplate for TypeMethodFnDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = TypeMethodFnEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        Ok(TypeMethodFnEtherealSignatureTemplate::new(db))
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_method_fn_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
) -> TypeMethodFnEtherealSignatureTemplate {
    todo!()
}
