use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ScriptItemPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ScriptItemPathData {
    pub script: Script,
}

impl ScriptItemPathData {
    pub fn module_path(self, db: &::salsa::Db) -> ScriptModulePath {
        self.script.module_path(db)
    }
}
