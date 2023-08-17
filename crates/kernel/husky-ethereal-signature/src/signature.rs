mod associated_item;
mod decr;
mod impl_block;
mod major_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::ty_variant::*;

use crate::*;
use husky_declarative_signature::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum ItemEtherealSignatureTemplate {
    Submodule,
    MajorItem(MajorItemEtherealSignatureTemplate),
    ImplBlock(ImplBlockEtherealSignatureTemplate),
    AssociatedItem(AssociatedItemEtherealSignatureTemplate),
    Variant(TypeVariantEtherealSignatureTemplate),
    Decr(DecrEtherealSignatureTemplate),
}

impl ItemEtherealSignatureTemplate {
    pub fn self_base_ty(self, db: &dyn EtherealSignatureDb) -> Option<EtherealTerm> {
        todo!()
        // match self {
        //     ItemEtherealSignatureTemplate::Submodule => None,
        //     ItemEtherealSignatureTemplate::MajorItem(_) => None,
        //     ItemEtherealSignatureTemplate::ImplBlock(template) => Some(template.self_base_ty(db)),
        //     ItemEtherealSignatureTemplate::AssociatedItem(template) => template.self_base_ty(db),
        //     ItemEtherealSignatureTemplate::Variant(template) => Some(template.self_base_ty(db)),
        //     ItemEtherealSignatureTemplate::Decr(_) => None,
        // }
    }

    pub fn self_value_ty(self, db: &dyn EtherealSignatureDb) -> Option<EtherealTerm> {
        match self {
            ItemEtherealSignatureTemplate::Submodule => None,
            ItemEtherealSignatureTemplate::MajorItem(_) => None,
            ItemEtherealSignatureTemplate::ImplBlock(template) => Some(template.self_value_ty(db)),
            ItemEtherealSignatureTemplate::AssociatedItem(template) => template.self_value_ty(db),
            ItemEtherealSignatureTemplate::Variant(template) => Some(template.self_value_ty(db)),
            ItemEtherealSignatureTemplate::Decr(_) => None,
        }
    }
}

pub trait HasEtherealSignatureTemplate {
    type EtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate>;
}

impl HasEtherealSignatureTemplate for ItemPath {
    type EtherealSignatureTemplate = ItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        Ok(match self {
            ItemPath::Submodule(_) => ItemEtherealSignatureTemplate::Submodule,
            ItemPath::MajorItem(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::AssociatedItem(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::TypeVariant(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.ethereal_signature_template(db)?.into(),
        })
    }
}
