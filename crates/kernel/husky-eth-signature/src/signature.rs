mod assoc_item;
mod attr;
mod impl_block;
mod major_item;
mod ty_variant;

pub use self::assoc_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::ty_variant::*;

use crate::*;
use husky_dec_signature::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ItemEthTemplate {
    Submodule,
    MajorItem(MajorItemEthTemplate),
    ImplBlock(ImplBlockEthTemplate),
    AssocItem(AssocItemEthTemplate),
    Variant(TypeVariantEthTemplate),
    Attr(AttrEthTemplate),
}

impl ItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            ItemEthTemplate::Submodule => None,
            ItemEthTemplate::MajorItem(_) => None,
            ItemEthTemplate::ImplBlock(template) => Some(template.self_ty(db)),
            ItemEthTemplate::AssocItem(template) => template.self_ty(db),
            ItemEthTemplate::Variant(template) => Some(template.self_ty(db)),
            ItemEthTemplate::Attr(_) => None,
        }
    }
}

pub trait HasEthTemplate {
    type EthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate>;
}

impl HasEthTemplate for ItemPath {
    type EthTemplate = ItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        Ok(match self {
            ItemPath::Submodule(_, _) => ItemEthTemplate::Submodule,
            ItemPath::MajorItem(path) => path.eth_template(db)?.into(),
            ItemPath::AssocItem(path) => path.eth_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.eth_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.eth_template(db)?.into(),
            ItemPath::Attr(_, path) => path.eth_template(db)?.into(),
        })
    }
}
