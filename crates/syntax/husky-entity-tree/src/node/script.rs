use super::*;
use husky_entity_path::path::script::ScriptItemPath;
use husky_vfs::{path::module_path::ScriptModulePath, script::Script};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
pub struct ScriptSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct ScriptSynNodePathData {
    script: Script,
}

impl ScriptSynNodePath {
    pub fn new(script: Script, db: &::salsa::Db) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ScriptSynNodePathData { script }.into(),
        ))
    }

    pub fn item_path(self) -> ScriptItemPath {
        // self.disambiguated_item_path.unambiguous_item_path()
        todo!()
    }
}

impl ScriptSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> ScriptSynNodePath {
        ScriptSynNodePath(id)
    }

    pub fn item_path(self) -> ScriptItemPath {
        // self.disambiguated_item_path.unambiguous_item_path()
        todo!()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ScriptModulePath {
        self.script.module_path(db)
    }
}
