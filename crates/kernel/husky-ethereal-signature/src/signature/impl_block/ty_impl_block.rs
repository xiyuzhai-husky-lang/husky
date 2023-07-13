use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeImplBlockEtherealSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: EtherealGenericParameters,
    pub self_ty: EtherealTerm,
}

impl HasEtherealSignatureTemplate for TypeImplBlockDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = TypeImplBlockEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_impl_block_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_impl_block_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature_template: TypeImplBlockDeclarativeSignatureTemplate,
) -> EtherealSignatureResult<TypeImplBlockEtherealSignatureTemplate> {
    let implicit_parameters = EtherealGenericParameters::from_declarative(
        db,
        declarative_signature_template.implicit_parameters(db),
    )?;
    let ty = EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty(db))?;
    Ok(TypeImplBlockEtherealSignatureTemplate::new(
        db,
        implicit_parameters,
        ty,
    ))
}
