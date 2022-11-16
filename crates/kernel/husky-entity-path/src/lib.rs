mod db;
mod display;
mod intern;
mod menu;

use std::path::PathBuf;

pub use db::*;
use husky_toolchain::Toolchain;
pub use menu::*;

use husky_word::Identifier;
pub use intern::*;
use optional::Optioned;
use semver::Version;
use url::Url;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePath {
    Builtin {
        ident: Identifier,
        toolchain: Toolchain,
    },
    Global {
        ident: Identifier,
        version: Version,
    },
    Local(PathBuf),
    Git(Url),
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
