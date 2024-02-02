use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitEthTemplate {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl HasEthTemplate for TraitPath {
    type EthTemplate = TraitEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        trai_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_ethereal_signature_template(
    db: &::salsa::Db,
    trai_path: TraitPath,
) -> EtherealSignatureResult<TraitEthTemplate> {
    let dec_template = trai_path.dec_template(db)?;
    let template_parameters =
        EthTemplateParameters::from_declarative(db, dec_template.template_parameters(db))?;
    Ok(TraitEthTemplate::new(db, trai_path, template_parameters))
}
