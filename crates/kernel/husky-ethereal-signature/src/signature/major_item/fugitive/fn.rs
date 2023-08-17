use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct FnFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
}

impl FnFugitiveEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: FugitivePath,
        declarative_signature_template: FnFugitiveDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        Ok(Self::new(db, path, template_parameters))
    }
}
