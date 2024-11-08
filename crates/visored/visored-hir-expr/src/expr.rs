pub mod application;
#[cfg(test)]
pub mod tests;

use crate::*;
use application::VdHirApplicationFunction;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use visored_opr::opr::binary::VdBaseBinaryOpr;
use visored_sem_expr::expr::{binary::VdSemBinaryDispatch, VdSemExprData, VdSemExprIdx};
use visored_zfc_ty::term::literal::VdZfcLiteral;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdHirExprData {
    Literal(VdZfcLiteral),
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

impl ToVdHir<VdHirExprIdx> for VdSemExprIdx {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirExprIdx {
        let data = builder.build_expr_from_sem_expr(self);
        builder.alloc_expr(data)
    }
}

impl<const N: usize> ToVdHir<VdHirExprIdxRange> for [VdSemExprIdx; N] {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirExprIdxRange {
        let data = self
            .into_iter()
            .map(|expr| builder.build_expr_from_sem_expr(expr))
            .collect::<Vec<_>>();
        builder.alloc_exprs(data)
    }
}

impl<'db> VdHirExprBuilder<'db> {
    fn build_expr_from_sem_expr(&mut self, sem_expr_idx: VdSemExprIdx) -> VdHirExprData {
        match self.sem_expr_arena()[sem_expr_idx] {
            VdSemExprData::Literal { literal, .. } => VdHirExprData::Literal(literal),
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
                arguments: [lopd, ropd].to_vd_hir(self),
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
            VdSemExprData::Attach { .. } => todo!(),
            VdSemExprData::UniadicChain => todo!(),
            VdSemExprData::VariadicChain => todo!(),
            VdSemExprData::UniadicArray => todo!(),
            VdSemExprData::VariadicArray => todo!(),
            VdSemExprData::Letter {
                token_idx_range,
                letter,
            } => todo!(),
            VdSemExprData::BaseOpr { opr } => todo!(),
            VdSemExprData::SeparatedList {
                separator,
                ref fragments,
            } => VdHirExprData::Application {
                function: todo!(),
                arguments: todo!(),
            },
            VdSemExprData::LxDelimited {
                left_delimiter_token_idx,
                item,
                right_delimiter_token_idx,
            } => todo!(),
            VdSemExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => todo!(),
            VdSemExprData::Fraction {
                command_token_idx,
                numerator_lcurl_token_idx,
                numerator,
                numerator_rcurl_token_idx,
                denominator_lcurl_token_idx,
                denominator,
                denominator_rcurl_token_idx,
            } => todo!(),
            VdSemExprData::Sqrt {
                command_token_idx,
                radicand_lcurl_token_idx,
                radicand,
                radicand_rcurl_token_idx,
            } => todo!(),
        }
    }
}
