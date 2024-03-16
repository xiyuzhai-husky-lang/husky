mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ImplBlockEthTemplate {
    TypeImpl(TypeImplBlockEthTemplate),
    TraitForTypeImpl(TraitForTypeImplBlockEthTemplate),
}

impl ImplBlockEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EthTerm {
        match self {
            ImplBlockEthTemplate::TypeImpl(template) => template.self_ty(db),
            ImplBlockEthTemplate::TraitForTypeImpl(template) => template.self_ty(db),
        }
    }
}

impl HasEthTemplate for ImplBlockPath {
    type EthTemplate = ImplBlockEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        Ok(match self {
            ImplBlockPath::TypeImplBlock(path) => path.eth_template(db)?.into(),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.eth_template(db)?.into(),
        })
    }
}
