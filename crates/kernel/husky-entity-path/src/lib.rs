mod db;
mod display;
mod intern;
mod menu;

pub use db::*;
pub use menu::*;

use husky_word::Identifier;
pub use intern::*;
use optional::Optioned;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CratePathKind {
    Library,
    Binary(Identifier),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PackagePath {
    version: (),
    kind: PackagePathKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PackagePathKind {
    Builtin,
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
