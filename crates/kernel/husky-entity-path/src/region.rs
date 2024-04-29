use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum RegionPath {
    Snippet(ScriptModulePath),
    Decl(ItemPath),
    Defn(ItemPath),
}

impl RegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            RegionPath::Snippet(slf) => slf.module_path(),
            RegionPath::Decl(slf) => slf.module_path(db),
            RegionPath::Defn(slf) => slf.module_path(db),
        }
    }
}
