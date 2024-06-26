use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitMethodRitchieEthTemplate {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: EthTemplateParameter,
}
