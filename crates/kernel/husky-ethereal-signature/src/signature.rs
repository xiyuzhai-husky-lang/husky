mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::ty_variant::*;

use crate::*;
use husky_declarative_signature::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
#[enum_class::from_variants]
pub enum ItemEtherealSignatureTemplate {
    Submodule,
    MajorItem(MajorItemEtherealSignatureTemplate),
    ImplBlock(ImplBlockEtherealSignatureTemplate),
    AssociatedItem(AssociatedItemEtherealSignatureTemplate),
    Variant(TypeVariantEtherealSignatureTemplate),
    Attr(AttrEtherealSignatureTemplate),
}

impl ItemEtherealSignatureTemplate {
    pub fn self_ty(self, db: &dyn EtherealSignatureDb) -> Option<EtherealTerm> {
        match self {
            ItemEtherealSignatureTemplate::Submodule => None,
            ItemEtherealSignatureTemplate::MajorItem(_) => None,
            ItemEtherealSignatureTemplate::ImplBlock(template) => Some(template.self_ty(db)),
            ItemEtherealSignatureTemplate::AssociatedItem(template) => template.self_ty(db),
            ItemEtherealSignatureTemplate::Variant(template) => Some(template.self_ty(db)),
            ItemEtherealSignatureTemplate::Attr(_) => None,
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
            ItemPath::Submodule(_, _) => ItemEtherealSignatureTemplate::Submodule,
            ItemPath::MajorItem(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::AssociatedItem(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.ethereal_signature_template(db)?.into(),
            ItemPath::Attr(_, path) => path.ethereal_signature_template(db)?.into(),
        })
    }
}
