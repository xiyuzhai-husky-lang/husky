use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum TexAstData {
    TextEdit { buffer: String },
    Other,
}

pub type TexAstArena = Arena<TexAstData>;
pub type TexAstIdx = ArenaIdx<TexAstData>;
pub type TexAstIdxRange = ArenaIdxRange<TexAstData>;
