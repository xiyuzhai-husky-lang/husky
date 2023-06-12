mod associated_item;
mod impl_block;
mod module_item;
mod ty_variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::ty_variant::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum EntityNodePath {
    ModuleItem(ModuleItemNodePath),
    TypeVariant(TypeVariantNodePath),
    ImplBlock(ImplBlockNodePath),
    AssociatedItem(AssociatedItemNodePath),
}

impl EntityNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            EntityNodePath::ModuleItem(node_path) => node_path.module_path(db),
            EntityNodePath::TypeVariant(node_path) => node_path.module_path(db),
            EntityNodePath::ImplBlock(node_path) => node_path.module_path(db),
            EntityNodePath::AssociatedItem(node_path) => node_path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

pub trait HasNodePath: Copy {
    type NodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath;
}

impl HasNodePath for EntityPath {
    type NodePath = EntityNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        match self {
            EntityPath::Module(path) => todo!(),
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::TypeVariant(_) => todo!(),
            EntityPath::ImplBlock(_) => todo!(),
        }
    }
}
