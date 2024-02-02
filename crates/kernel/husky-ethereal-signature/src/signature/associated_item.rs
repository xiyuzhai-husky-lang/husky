mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum AssociatedItemEthTemplate {
    TraitForType(TraitForTypeItemEthTemplate),
    Type(TypeItemEthTemplate),
    Trait(TraitItemEthTemplate),
}

impl AssociatedItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            AssociatedItemEthTemplate::TraitForType(template) => template.self_ty(db),
            AssociatedItemEthTemplate::Type(template) => template.self_ty(db),
            AssociatedItemEthTemplate::Trait(template) => template.self_ty(db),
        }
    }
}

impl HasEthTemplate for AssociatedItemPath {
    type EthTemplate = AssociatedItemEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        match self {
            AssociatedItemPath::TypeItem(path) => {
                path.ethereal_signature_template(db).map(Into::into)
            }
            AssociatedItemPath::TraitItem(path) => {
                path.ethereal_signature_template(db).map(Into::into)
            }
            AssociatedItemPath::TraitForTypeItem(path) => {
                path.ethereal_signature_template(db).map(Into::into)
            }
        }
    }
}
