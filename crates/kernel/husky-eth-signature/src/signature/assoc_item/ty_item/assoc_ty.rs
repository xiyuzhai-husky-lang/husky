use super::*;
use husky_entity_path::path::assoc_item::trai_for_ty_item::TraitForTypeItemPath;

#[salsa::interned]
pub struct TypeAssocTypeEthTemplate {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub assoc_ty: EthTerm,
}
