use super::*;
use husky_entity_path::path::major_item::trai::TraitPath;

#[salsa::interned]
pub struct TraitEthTemplate {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl HasEthTemplate for TraitPath {
    type EthTemplate = TraitEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        trai_eth_template(db, self)
    }
}

#[salsa::tracked]
fn trai_eth_template(
    db: &::salsa::Db,
    trai_path: TraitPath,
) -> EthSignatureResult<TraitEthTemplate> {
    let dec_template = trai_path.dec_template(db)?;
    let template_parameters =
        EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
    Ok(TraitEthTemplate::new(db, trai_path, template_parameters))
}
