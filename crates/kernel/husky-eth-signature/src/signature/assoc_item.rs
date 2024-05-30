pub mod trai_for_ty_item;
pub mod trai_item;
pub mod ty_item;

use self::trai_for_ty_item::*;
use self::trai_item::*;
use self::ty_item::*;
use super::*;
use husky_entity_path::path::assoc_item::AssocItemPath;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum AssocItemEthTemplate {
    TraitForType(TraitForTypeItemEthTemplate),
    Type(TypeItemEthTemplate),
    Trait(TraitItemEthTemplate),
}

impl AssocItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> AssocItemPath {
        match self {
            AssocItemEthTemplate::TraitForType(slf) => slf.path(db).into(),
            AssocItemEthTemplate::Type(slf) => slf.path(db).into(),
            AssocItemEthTemplate::Trait(slf) => slf.path(db).into(),
        }
    }

    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            AssocItemEthTemplate::TraitForType(slf) => slf.self_ty(db),
            AssocItemEthTemplate::Type(slf) => slf.self_ty(db),
            AssocItemEthTemplate::Trait(slf) => slf.self_ty(db),
        }
    }
}

impl HasEthTemplate for AssocItemPath {
    type EthTemplate = AssocItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        match self {
            AssocItemPath::TypeItem(path) => path.eth_template(db).map(Into::into),
            AssocItemPath::TraitItem(path) => path.eth_template(db).map(Into::into),
            AssocItemPath::TraitForTypeItem(path) => path.eth_template(db).map(Into::into),
        }
    }
}
