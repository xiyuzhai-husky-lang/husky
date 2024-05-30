pub mod assoc_item;
pub mod attr;
pub mod crate_;
pub mod impl_block;
pub mod major_item;
pub mod ty_variant;

use self::assoc_item::*;
use self::attr::*;
use self::impl_block::*;
use self::major_item::*;
use self::ty_variant::*;
use crate::parameter::EtherealParenateParameters;
use crate::*;
use husky_dec_signature::signature::{HasDecSignature, HasDecTemplate};
use husky_entity_path::path::{submodule::SubmoduleItemPath, ItemPath};
use husky_eth_term::fmt::with_eth_term_fmt_context;
use salsa::fmt::WithFmtContext;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ItemEthTemplate {
    Submodule(SubmoduleItemPath),
    MajorItem(MajorItemEthTemplate),
    ImplBlock(ImplBlockEthTemplate),
    AssocItem(AssocItemEthTemplate),
    Variant(TypeVariantEthTemplate),
    Attr(AttrEthTemplate),
}

impl ItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> ItemPath {
        match self {
            ItemEthTemplate::Submodule(slf) => slf.into(),
            ItemEthTemplate::MajorItem(slf) => slf.path(db).into(),
            ItemEthTemplate::ImplBlock(slf) => slf.path(db).into(),
            ItemEthTemplate::AssocItem(slf) => slf.path(db).into(),
            ItemEthTemplate::Variant(slf) => slf.path(db).into(),
            ItemEthTemplate::Attr(slf) => slf.path(db).into(),
        }
    }

    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            ItemEthTemplate::Submodule(_) => None,
            ItemEthTemplate::MajorItem(_) => None,
            ItemEthTemplate::ImplBlock(template) => Some(template.self_ty(db)),
            ItemEthTemplate::AssocItem(template) => template.self_ty(db),
            ItemEthTemplate::Variant(template) => Some(template.self_ty(db)),
            ItemEthTemplate::Attr(_) => None,
        }
    }
}

impl WithFmtContext for ItemEthTemplate {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> std::fmt::Result,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        let ctx = item_fmt_context(db, *self.path(db));
        with_eth_term_fmt_context(ctx, f, db)
    }
}

pub trait HasEthTemplate {
    type EthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate>;
}

impl HasEthTemplate for ItemPath {
    type EthTemplate = ItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        Ok(match self {
            ItemPath::Submodule(_, path) => ItemEthTemplate::Submodule(path),
            ItemPath::MajorItem(path) => path.eth_template(db)?.into(),
            ItemPath::AssocItem(path) => path.eth_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.eth_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.eth_template(db)?.into(),
            ItemPath::Attr(_, path) => path.eth_template(db)?.into(),
            ItemPath::Script(_, _) => todo!(),
        })
    }
}
