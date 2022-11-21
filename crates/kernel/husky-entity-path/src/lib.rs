mod crate_path;
mod db;
mod display;
mod menu;
mod package_path;

pub use db::*;
pub use menu::*;
pub use package_path::{PackagePath, PackagePathData};

use crate_path::CratePathKind;
use husky_identifier::Identifier;
use husky_toolchain::Toolchain;
use optional::Optioned;
use salsa::DbWithJar;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(PackagePath, EntityPath);

#[salsa::interned(jar = EntityPathJar)]
pub struct EntityPath {
    #[return_ref]
    variant: EntityPathVariant,
    ident: Identifier,
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
}
