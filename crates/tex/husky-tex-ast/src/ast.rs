use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum MathAstData {
    TextEdit { buffer: String },
    Other,
}

pub type MathAstArena = Arena<MathAstData>;
pub type MathAstIdx = ArenaIdx<MathAstData>;
pub type MathAstIdxRange = ArenaIdxRange<MathAstData>;
