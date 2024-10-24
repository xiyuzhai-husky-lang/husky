pub mod application;

use crate::*;
use application::VdHirApplicationFunction;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use visored_opr::opr::binary::VdBinaryOpr;
use visored_sem_expr::expr::{binary::VdSemBinaryDispatch, VdSemExprData, VdSemExprIdx};
use visored_zfs_ty::term::literal::VdZfsLiteral;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdHirExprData {
    Literal(VdZfsLiteral),
    Variable(VdHirVariable),
    Application {
        function: VdHirApplicationFunction,
        arguments: VdHirExprIdxRange,
    },
}

pub type VdHirExprArena = Arena<VdHirExprData>;
pub type VdHirExprArenaRef<'a> = ArenaRef<'a, VdHirExprData>;
pub type VdHirExprIdx = ArenaIdx<VdHirExprData>;
pub type VdHirExprIdxRange = ArenaIdxRange<VdHirExprData>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdHirLiteral {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdHirVariable {}

impl ToVdHir for VdSemExprIdx {
    type Output = VdHirExprIdx;

    fn to_hir(self, builder: &mut VdHirExprBuilder) -> Self::Output {
        let data = builder.build_expr_from_sem_expr(self);
        builder.alloc_expr(data)
    }
}

impl<const N: usize> ToVdHir for [VdSemExprIdx; N] {
    type Output = VdHirExprIdxRange;

    fn to_hir(self, builder: &mut VdHirExprBuilder) -> Self::Output {
        let data = self
            .into_iter()
            .map(|expr| builder.build_expr_from_sem_expr(expr))
            .collect::<Vec<_>>();
        builder.alloc_exprs(data)
    }
}

impl<'db> VdHirExprBuilder<'db> {
    fn build_expr_from_sem_expr(&mut self, sem_expr_idx: VdSemExprIdx) -> VdHirExprData {
        match self.vd_sem_expr_arena()[sem_expr_idx] {
            VdSemExprData::Literal {
                literal,
                ref dispatch,
            } => VdHirExprData::Literal(literal),
            VdSemExprData::Notation => todo!(),
            VdSemExprData::Binary {
                lopd,
                opr,
                ropd,
                ref dispatch,
            } => VdHirExprData::Application {
                function: match dispatch {
                    VdSemBinaryDispatch::IntAdd => VdHirApplicationFunction::IntAdd,
                    VdSemBinaryDispatch::TrivialEq => VdHirApplicationFunction::TrivialEq,
                },
                arguments: [lopd, ropd].to_hir(self),
            },
            VdSemExprData::Prefix {
                opr,
                opd,
                ref dispatch,
            } => todo!(),
            VdSemExprData::Suffix {
                opd,
                opr,
                ref dispatch,
            } => todo!(),
            VdSemExprData::Attach {
                base,
                top,
                bottom,
                top_left,
                bottom_left,
                top_right,
                bottom_right,
                ref dispatch,
            } => todo!(),
            VdSemExprData::UniadicChain => todo!(),
            VdSemExprData::VariadicChain => todo!(),
            VdSemExprData::UniadicArray => todo!(),
            VdSemExprData::VariadicArray => todo!(),
        }
    }
}
