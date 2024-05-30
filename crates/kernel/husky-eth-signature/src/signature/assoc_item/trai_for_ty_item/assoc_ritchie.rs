use super::*;
use husky_dec_signature::signature::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieDecTemplate;

#[salsa::interned]
pub struct TraitForTypeAssocRitchieEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TraitForTypeAssocRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_template: TraitForTypeAssocRitchieDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, dec_template.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, dec_template.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}
