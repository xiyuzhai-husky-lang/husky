mod crate_path;
mod db;
mod display;
mod intern;
mod menu;
mod package_path;

pub use db::*;
use husky_toolchain::Toolchain;
pub use intern::*;
pub use menu::*;

use crate_path::CratePathKind;
use husky_word::Identifier;
use optional::Optioned;
use package_path::PackagePathVariant;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityPath {
    variant: EntityPathVariant,
    ident: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityPathVariant {
    Crate {
        package: PackagePathVariant,
        kind: CratePathKind,
    },
    Childpath {
        parent: EntityPathItd,
    },
}

impl EntityPath {
    pub fn root(ident: Identifier) -> Self {
        Self {
            ident,
            variant: EntityPathVariant::Crate {
                package: PackagePathVariant::Builtin {
                    toolchain: Toolchain::new_ad_hoc(),
                },
                kind: CratePathKind::Library,
            },
        }
    }

    #[inline(always)]
    pub fn opt_parent(&self) -> Option<EntityPathItd> {
        match self.variant {
            EntityPathVariant::Crate { .. } => None,
            EntityPathVariant::Childpath { parent, .. } => Some(parent),
        }
    }
}
