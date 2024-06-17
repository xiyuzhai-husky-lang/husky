use super::*;
use husky_entity_path::path::chunk::ChunkItemPath;
use husky_vfs::{chunk::Chunk, path::module_path::ChunkModulePath};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
pub struct ChunkSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct ChunkSynNodePathData {
    chunk: Chunk,
}

impl ChunkSynNodePath {
    pub fn new(chunk: Chunk, db: &::salsa::Db) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ChunkSynNodePathData { chunk }.into(),
        ))
    }

    pub fn item_path(self) -> ChunkItemPath {
        // self.disambiguated_item_path.unambiguous_item_path()
        todo!()
    }
}

impl ChunkSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> ChunkSynNodePath {
        ChunkSynNodePath(id)
    }

    pub fn item_path(self) -> ChunkItemPath {
        // self.disambiguated_item_path.unambiguous_item_path()
        todo!()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ChunkModulePath {
        self.chunk.module_path(db)
    }
}
