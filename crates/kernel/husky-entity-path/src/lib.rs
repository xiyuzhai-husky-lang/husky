mod crate_path;
mod db;
mod display;
mod intern;
mod menu;
mod package_path;

pub use db::*;
pub use intern::*;
pub use menu::*;

use crate_path::CratePathKind;
use husky_word::Identifier;
use optional::Optioned;
use package_path::PackagePath;

// EntityPath examples: std::ops::Add

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityPath {
    Crate {
        package: PackagePath,
        kind: CratePathKind,
    },
    Childpath {
        parent: EntityPathItd,
        ident: Identifier,
    },
}

impl EntityPath {
    pub fn root(ident: Identifier) -> Self {
        todo!()
        // Self {
        //     opt_parent: Optioned::none(),
        //     ident,
        // }
    }

    #[inline(always)]
    pub fn opt_parent(&self) -> Option<EntityPathItd> {
        todo!()
        // self.opt_parent.into_option()
    }
}
