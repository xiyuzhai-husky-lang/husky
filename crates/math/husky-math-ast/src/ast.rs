use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum MathAstData {
    TextEdit {
        kind: MathAstTextEditKind,
        buffer: String,
    },
    Other,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MathAstTextEditKind {
    Latex,
    Typst,
}

pub type MathAstArena = Arena<MathAstData>;
pub type MathAstIdx = ArenaIdx<MathAstData>;
pub type MathAstIdxRange = ArenaIdxRange<MathAstData>;
