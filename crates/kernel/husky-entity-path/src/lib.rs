#![feature(trait_upcasting)]
mod ancestry;
mod db;
mod jar;
mod menu;
#[cfg(test)]
mod tests;
mod utils;

pub use db::*;
pub use jar::*;
pub use menu::*;

use ancestry::*;
use husky_package_path::*;
use husky_toolchain::Toolchain;
use husky_word::Identifier;
use optional::Optioned;
use salsa::DbWithJar;
use utils::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct EntityPath {
    pub data: EntityPathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPathData {
    CrateRoot(CratePath),
    Childpath {
        parent: EntityPath,
        ident: Identifier,
    },
}

impl EntityPath {
    pub fn root(ident: Identifier) -> Self {
        todo!()
        // Self {
        //     ident,
        //     variant: EntityPathVariant::Crate {
        //         package: PackagePathData::Builtin {
        //             toolchain: Toolchain::new_ad_hoc(),
        //         },
        //         kind: CratePathKind::Library,
        //     },
        // }
    }

    pub fn new_crate_root(
        db: &dyn EntityPathDb,
        package_path: PackagePath,
        crate_kind: CrateKind,
    ) -> Self {
        db.it_entity_path(EntityPathData::CrateRoot(CratePath::new(
            db,
            package_path,
            crate_kind,
        )))
    }

    #[inline(always)]
    pub fn opt_parent(&self) -> Option<EntityPath> {
        todo!()
        // match self.variant {
        //     EntityPathVariant::Crate { .. } => None,
        //     EntityPathVariant::Childpath { parent, .. } => Some(parent),
        // }
    }

    pub(crate) fn child(self, db: &dyn EntityPathDb, ident: &str) -> Option<EntityPath> {
        db.it_child_entity_path(self, ident)
    }

    pub fn show(self, db: &dyn EntityPathDb) -> String {
        match self.data(db) {
            EntityPathData::CrateRoot(crate_path) => {
                format!("crate({:?})", crate_path.package_path(db).data(db))
            }
            EntityPathData::Childpath { parent, ident } => {
                format!("{}::{}", parent.show(db), db.dt_ident(ident))
            }
        }
    }
}
