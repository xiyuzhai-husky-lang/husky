mod ancestry;
mod crate_path;
mod db;
mod jar;
mod menu;
#[cfg(test)]
mod tests;
mod utils;

pub use crate_path::CratePathKind;
pub use db::*;
pub use jar::*;
pub use menu::*;

use ancestry::*;
use husky_package_path::PackagePath;
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
    Crate {
        package: PackagePath,
        kind: CratePathKind,
    },
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

    pub fn display(self, db: &dyn EntityPathDb) -> String {
        match self.data(db) {
            EntityPathData::Crate { package, kind } => "crate".into(),
            EntityPathData::Childpath { parent, ident } => {
                format!("{}::{}", parent.display(db), db.dt_ident(ident))
            }
        }
    }
}
