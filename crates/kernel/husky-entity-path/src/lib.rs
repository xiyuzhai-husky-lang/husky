#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod associated_item;
mod db;
mod error;
mod menu;
mod module_item;
#[cfg(test)]
mod tests;
mod utils;

pub use ancestry::*;
pub use associated_item::*;
pub use db::*;
pub use error::*;
pub use menu::*;
pub use module_item::*;

use husky_vfs::*;
use husky_word::Identifier;

use salsa::DbWithJar;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(ModuleItemPath, AssociatedItemPath, entity_path_menu);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    AssociatedItem(AssociatedItemPath),
}

impl 
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

//     pub fn apparent_ancestry(self, db: &dyn EntityPathDb) -> &EntityAncestry {
//         entity_apparent_ancestry(db, self)
//     }

//     pub fn apparent_crate_path(self, db: &dyn EntityPathDb) -> CratePath {
//         self.apparent_ancestry(db).crate_path()
//     }
// }
