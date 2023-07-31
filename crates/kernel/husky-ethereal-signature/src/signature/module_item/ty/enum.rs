use super::*;
use husky_declarative_signature::EnumTypeDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
}

impl EnumTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypePath,
        declarative_signature_template: EnumTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        Ok(Self::new(db, path, template_parameters))
    }
}
