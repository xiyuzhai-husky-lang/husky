use crate::{path::ItemPath, *};

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
}
