use super::*;
use husky_dec_signature::signature::assoc_item::trai_item::assoc_ty::TraitAssocTypeDecTemplate;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocTypeEthTemplate {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl TraitAssocTypeEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitItemPath,
        tmpl: TraitAssocTypeDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
