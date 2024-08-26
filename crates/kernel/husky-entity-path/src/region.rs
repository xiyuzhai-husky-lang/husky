use crate::{path::ItemPath, *};
use husky_vfs::{
    path::{
        crate_path::CratePath,
        module_path::{ModulePath, ScriptModulePath},
    },
    toolchain::Toolchain,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum RegionPath {
    CrateDecl(CratePath),
    ItemDecl(ItemPath),
    ItemDefn(ItemPath),
    Script(ScriptModulePath),
}

impl RegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            RegionPath::CrateDecl(slf) => slf.root_module_path(db),
            RegionPath::ItemDecl(slf) => slf.module_path(db),
            RegionPath::ItemDefn(slf) => slf.module_path(db),
            RegionPath::Script(slf) => slf.module_path(),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self {
            RegionPath::CrateDecl(slf) => slf.toolchain(db),
            RegionPath::ItemDecl(slf) => slf.toolchain(db),
            RegionPath::ItemDefn(slf) => slf.toolchain(db),
            RegionPath::Script(slf) => slf.toolchain(db),
        }
    }
}
