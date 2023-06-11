mod associated_item;
mod impl_block;
mod module_item;
mod variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(TypeVariantPath),
    ImplBlock(ImplBlockPath),
}
