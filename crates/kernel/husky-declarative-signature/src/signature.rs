mod associated_item;
mod attr;
mod impl_block;
mod module_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::ty_variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum SignatureTemplate {
    Submodule,
    MajorItem(MajorItemDeclarativeSignatureTemplate),
    ImplBlock(ImplBlockDeclarativeSignatureTemplate),
    AssociatedItem(AssociatedItemDeclarativeSignatureTemplate),
    Variant(TypeVariantDeclarativeSignatureTemplate),
    Attr(AttrDeclarativeSignatureTemplate),
}

pub trait HasDeclarativeSignatureTemplate: Copy {
    type DeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate>;
}

impl HasDeclarativeSignatureTemplate for ItemPath {
    type DeclarativeSignatureTemplate = SignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        Ok(match self {
            ItemPath::Submodule(_, _) => SignatureTemplate::Submodule,
            ItemPath::MajorItem(path) => path.declarative_signature_template(db)?.into(),
            ItemPath::AssociatedItem(path) => path.declarative_signature_template(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.declarative_signature_template(db)?.into(),
            ItemPath::ImplBlock(path) => path.declarative_signature_template(db)?.into(),
            ItemPath::Attr(_, _) => todo!(),
        })
    }
}
