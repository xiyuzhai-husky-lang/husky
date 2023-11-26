mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
#[enum_class::from_variants]
pub enum AssociatedItemEtherealSignatureTemplate {
    TraitForType(TraitForTypeItemEtherealSignatureTemplate),
    Type(TypeItemEtherealSignatureTemplate),
    Trait(TraitItemEtherealSignatureTemplate),
}

impl AssociatedItemEtherealSignatureTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            AssociatedItemEtherealSignatureTemplate::TraitForType(template) => {
                template.self_value_ty(db)
            }
            AssociatedItemEtherealSignatureTemplate::Type(template) => template.self_ty(db),
            AssociatedItemEtherealSignatureTemplate::Trait(template) => template.self_ty(db),
        }
    }
}

impl HasEtherealSignatureTemplate for AssociatedItemPath {
    type EtherealSignatureTemplate = AssociatedItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
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
