use crate::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use visored_sem_expr::expr::VdSemExprIdx;

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

impl ToVdHir for VdSemExprIdx {
    type Output = VdHirExprIdx;

    fn to_hir(self, builder: &mut VdHirExprBuilder) -> Self::Output {
        todo!()
    }
}
