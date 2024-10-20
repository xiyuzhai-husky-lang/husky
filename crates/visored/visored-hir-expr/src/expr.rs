use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdHirExprData {
    Literal(VdHirLiteral),
    Variable(VdHirVariable),
    Application {
        function: VdHirExprIdx,
        arguments: VdHirExprIdxRange,
    },
}

pub type VdHirExprArena = Arena<VdHirExprData>;
pub type VdHirExprIdx = ArenaIdx<VdHirExprData>;
pub type VdHirExprIdxRange = ArenaIdxRange<VdHirExprData>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdHirLiteral {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdHirVariable {}
