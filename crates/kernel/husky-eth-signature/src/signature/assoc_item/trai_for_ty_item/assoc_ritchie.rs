use super::*;

#[salsa::interned]
pub struct TraitForTypeAssocRitchieEthTemplate {
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}
