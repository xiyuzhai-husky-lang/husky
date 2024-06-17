use crate::*;
use husky_vfs::path::module_path::ChunkModulePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxBase {
    /// equal to the value of TokenIdx::index on the starting token
    index_base: usize,
    len: usize,
}

/// # constructors
impl RegionalTokenIdxBase {
    pub fn new_chunk(chunk_module_path: ChunkModulePath, db: &::salsa::Db) -> Self {
        let chunk = chunk_module_path.chunk(db);
        Self {
            index_base: 0,
            len: db.chunk_token_sheet_data(chunk).len(),
        }
    }

    pub fn new(token_verse_base: TokenVerseStart, tokens: &Vec<TokenData>) -> Self {
        Self {
            index_base: token_verse_base.token_idx().index(),
            len: tokens.len(),
        }
    }
}

/// # getters
impl RegionalTokenIdxBase {
    /// equal to the value of TokenIdx::index on the starting token
    pub fn index_base(&self) -> usize {
        self.index_base
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn token_idx_range(self) -> TokenIdxRange {
        let start = self.index_base;
        let end = start + self.len;
        let t = TokenIdx::from_usize_index_ext;
        unsafe { TokenIdxRange::new(t(start), t(end)) }
    }
}
