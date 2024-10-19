use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredHirExprData {
    Literal(VisoredHirLiteral),
    Variable(VisoredHirVariable),
    Application {
        function: VisoredHirExprIdx,
        arguments: VisoredHirExprIdxRange,
    },
}

pub type VisoredHirExprArena = Arena<VisoredHirExprData>;
pub type VisoredHirExprIdx = ArenaIdx<VisoredHirExprData>;
pub type VisoredHirExprIdxRange = ArenaIdxRange<VisoredHirExprData>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VisoredHirLiteral {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VisoredHirVariable {}
