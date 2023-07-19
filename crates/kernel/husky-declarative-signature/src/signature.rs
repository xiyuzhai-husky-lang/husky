mod associated_item;
mod decr;
mod impl_block;
mod module_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::ty_variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum SignatureTemplate {
    Module,
    ModuleItem(ModuleItemDeclarativeSignatureTemplate),
    ImplBlock(ImplBlockDeclarativeSignatureTemplate),
    AssociatedItem(AssociatedItemDeclarativeSignatureTemplate),
    Variant(TypeVariantDeclarativeSignatureTemplate),
    Decr(DecrSignatureTemplate),
}

pub trait HasDeclarativeSignatureTemplate: Copy {
    type DeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate>;
}

impl HasDeclarativeSignatureTemplate for EntityPath {
    type DeclarativeSignatureTemplate = SignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        Ok(match self {
            EntityPath::Module(_) => SignatureTemplate::Module,
            EntityPath::ModuleItem(path) => path.declarative_signature_template(db)?.into(),
            EntityPath::AssociatedItem(path) => path.declarative_signature_template(db)?.into(),
            EntityPath::TypeVariant(path) => path.declarative_signature_template(db)?.into(),
            EntityPath::ImplBlock(path) => path.declarative_signature_template(db)?.into(),
        })
    }
}
