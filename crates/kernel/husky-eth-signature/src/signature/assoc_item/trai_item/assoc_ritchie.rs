use super::*;

#[salsa::interned]
pub struct TraitAssocRitchieEthTemplate {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}
