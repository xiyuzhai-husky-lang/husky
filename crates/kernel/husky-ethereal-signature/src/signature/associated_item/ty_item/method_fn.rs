use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
}

impl HasEtherealSignatureTemplate for TypeMethodFnDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = TypeMethodFnEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        let self_ty = EtherealTerm::ty_from_declarative(db, self.self_ty(db))?;
        Ok(TypeMethodFnEtherealSignatureTemplate::new(db, self_ty))
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_method_fn_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
) -> TypeMethodFnEtherealSignatureTemplate {
    todo!()
}
