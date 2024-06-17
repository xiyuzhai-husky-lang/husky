use super::*;
use husky_vfs::path::module_path::ChunkModulePath;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ChunkItemPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ChunkItemPathData {
    pub chunk: Chunk,
}

impl ChunkItemPathData {
    pub fn module_path(self, db: &::salsa::Db) -> ChunkModulePath {
        self.chunk.module_path(db)
    }
}
