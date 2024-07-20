use super::*;
use crate::signature::impl_block::ty_impl_block::TypeImplBlockEthTemplate;
use husky_dec_signature::signature::assoc_item::trai_item::memo::TraitMemoizedFieldDecTemplate;
use package::PackageEthSignatureData;

#[salsa::interned]
pub struct TraitMemoizedFieldEthTemplate {
    pub path: TraitItemPath,
    pub impl_block: TypeImplBlockEthTemplate,
    pub return_ty: EthTerm,
}

/// # getters
impl TraitMemoizedFieldEthTemplate {
    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        &[]
    }
}
