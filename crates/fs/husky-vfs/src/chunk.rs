//! the term `chunk` is borrowed from lua
use crate::*;

#[salsa::input]
pub struct Chunk {
    pub source: ScriptSource,
    #[return_ref]
    pub data: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptSource {
    Snippet { toolchain: Toolchain },
    Child { parent: Chunk },
}

impl Chunk {
    #[cfg(feature = "test_utils")]
    pub fn new_dev_snippet(data: impl Into<String>, db: &::salsa::Db) -> Self {
        let toolchain = db.dev_toolchain().unwrap();
        Self::new(db, ScriptSource::Snippet { toolchain }, data.into())
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        chunk_toolchain(db, self)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ChunkModulePath {
        ChunkModulePath::new(self, db)
    }
}

#[salsa::tracked]
fn chunk_toolchain(db: &::salsa::Db, chunk: Chunk) -> Toolchain {
    match chunk.source(db) {
        ScriptSource::Snippet { toolchain } => toolchain,
        ScriptSource::Child { parent } => parent.toolchain(db),
    }
}
