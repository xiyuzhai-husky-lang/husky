mod crate_path;
mod db;
mod display;
mod jar;
mod menu;
mod package_path;

pub use db::*;
pub use jar::*;
pub use menu::*;
pub use package_path::{PackagePath, PackagePathData};

use crate_path::CratePathKind;
use husky_identifier::Identifier;
use husky_toolchain::Toolchain;
use optional::Optioned;
use salsa::DbWithJar;

#[salsa::interned(jar = EntityPathJar)]
pub struct EntityPath {
    ident: Identifier,
    #[return_ref]
    variant: EntityPathVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EntityPathVariant {
    Crate {
        package: PackagePathData,
        kind: CratePathKind,
    },
    Childpath {
        parent: EntityPath,
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

    pub(crate) fn child(self, db: &dyn EntityPathDb, ident: &str) -> EntityPath {
        db.it_child_entity_path(self, ident)
    }
}
