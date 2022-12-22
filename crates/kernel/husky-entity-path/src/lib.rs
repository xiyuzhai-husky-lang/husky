#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod associated_item;
mod db;
mod enum_variant;
mod error;
mod menu;
mod module_item;
#[cfg(test)]
mod tests;
mod utils;

pub use ancestry::*;
pub use associated_item::*;
pub use db::*;
pub use enum_variant::*;
pub use error::*;
pub use menu::*;
pub use module_item::*;

use husky_vfs::*;
use husky_word::Identifier;
use salsa::DbWithJar;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    ModuleItemPath,
    AssociatedItemPath,
    ModuleItemVariantPath,
    entity_path_menu,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    AssociatedItem(AssociatedItemPath),
    EnumVariant(ModuleItemVariantPath),
}

impl EntityPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Identifier {
        match self {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::EnumVariant(_) => todo!(),
        }
    }
}
// impl EntityPath {
//     pub fn new_module(db: &dyn EntityPathDb, module_path: ModulePath) -> Self {
//         EntityPath::new(db, EntityPathData::Module(module_path))
//     }

//     pub fn new_child(self, db: &dyn EntityPathDb, ident: &str) -> Option<EntityPath> {
//         let ident = db.it_ident_borrowed(ident)?;
//         Some(EntityPath::new(
//             db,
//             EntityPathData::Associated {
//                 parent: self,
//                 ident,
//             },
//         ))
//     }

//     pub fn show(self, db: &dyn EntityPathDb) -> String {
//         match self.data(db) {
//             EntityPathData::Module(_crate_path) => "crate".to_owned(),
//             EntityPathData::Associated { parent, ident } => {
//                 format!("{}::{}", parent.show(db), db.dt_ident(ident))
//             }
//         }
//     }

//     pub fn module_ancestry(self, db: &dyn EntityPathDb) -> &EntityAncestry {
//         entity_module_ancestry(db, self)
//     }

//     pub fn apparent_crate_path(self, db: &dyn EntityPathDb) -> CratePath {
//         self.module_ancestry(db).crate_path()
//     }
// }

impl salsa::DebugWithDb<dyn EntityPathDb + '_> for EntityPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            EntityPath::Module(module_path) => f
                .debug_tuple("Module")
                .field(&module_path.debug_with(db as &dyn VfsDb, include_all_fields))
                .finish(),
            EntityPath::ModuleItem(module_item_path) => f
                .debug_tuple("ModuleItem")
                .field(&module_item_path.debug_with(db, include_all_fields))
                .finish(),
            EntityPath::AssociatedItem(associated_item_path) => f
                .debug_tuple("AssociatedItem")
                .field(&associated_item_path.debug_with(db, include_all_fields))
                .finish(),
            EntityPath::EnumVariant(enum_variant_path) => f
                .debug_tuple("EnumVariantItem")
                .field(&enum_variant_path.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}

impl<Db> salsa::DebugWithDb<Db> for EntityPath
where
    Db: EntityPathDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityPathDb, include_all_fields)
    }
}

impl From<ModuleItemVariantPath> for EntityPath {
    fn from(v: ModuleItemVariantPath) -> Self {
        Self::EnumVariant(v)
    }
}

impl From<AssociatedItemPath> for EntityPath {
    fn from(v: AssociatedItemPath) -> Self {
        Self::AssociatedItem(v)
    }
}

impl From<ModuleItemPath> for EntityPath {
    fn from(v: ModuleItemPath) -> Self {
        Self::ModuleItem(v)
    }
}

impl From<ModulePath> for EntityPath {
    fn from(v: ModulePath) -> Self {
        Self::Module(v)
    }
}
