pub mod application;
#[cfg(test)]
pub mod tests;

use crate::*;
use application::VdMirApplicationFunction;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use visored_global_resolution::resolution::letter::VdLetterGlobalResolution;
use visored_item_path::path::VdItemPath;
use visored_opr::opr::binary::VdBaseBinaryOpr;
use visored_sem_expr::expr::{
    binary::VdSemBinaryDispatch, letter::VdSemLetterDispatch,
    separated_list::VdSemSeparatedListDispatch, VdSemExprData, VdSemExprIdx, VdSemExprIdxRange,
};
use visored_zfc_ty::term::literal::VdZfcLiteral;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VdMirExprData {
    Literal(VdZfcLiteral),
    Variable(VdMirVariable),
    Application {
        function: VdMirApplicationFunction,
        arguments: VdMirExprIdxRange,
    },
    ItemPath(VdItemPath),
}

pub type VdMirExprArena = Arena<VdMirExprData>;
pub type VdMirExprArenaRef<'a> = ArenaRef<'a, VdMirExprData>;
pub type VdMirExprIdx = ArenaIdx<VdMirExprData>;
pub type VdMirExprIdxRange = ArenaIdxRange<VdMirExprData>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdMirLiteral {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdMirVariable {}

impl ToVdMir<VdMirExprIdxRange> for VdSemExprIdxRange {
    fn to_vd_hir(self, builder: &mut VdMirExprBuilder) -> VdMirExprIdxRange {
        let mut exprs: Vec<VdMirExprData> = Vec::with_capacity(self.len());
        for expr in self {
            exprs.push(builder.build_expr(expr));
        }
        builder.alloc_exprs(exprs)
    }
}

impl ToVdMir<VdMirExprIdx> for VdSemExprIdx {
    fn to_vd_hir(self, builder: &mut VdMirExprBuilder) -> VdMirExprIdx {
        let data = builder.build_expr(self);
        builder.alloc_expr(data)
    }
}

impl<const N: usize> ToVdMir<VdMirExprIdxRange> for [VdSemExprIdx; N] {
    fn to_vd_hir(self, builder: &mut VdMirExprBuilder) -> VdMirExprIdxRange {
        let data = self
            .into_iter()
            .map(|expr| builder.build_expr(expr))
            .collect::<Vec<_>>();
        builder.alloc_exprs(data)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_expr(&mut self, sem_expr_idx: VdSemExprIdx) -> VdMirExprData {
        match *self.sem_expr_arena()[sem_expr_idx].data() {
            VdSemExprData::Literal { literal, .. } => VdMirExprData::Literal(literal),
            VdSemExprData::Binary {
                lopd,
                opr,
                ropd,
                ref dispatch,
            } => VdMirExprData::Application {
                function: match dispatch {
                    VdSemBinaryDispatch::IntAdd => todo!(),
                    VdSemBinaryDispatch::TrivialEq => todo!(),
                    // VdSemBinaryDispatch::IntAdd => VdMirApplicationFunction::IntAdd,
                    // VdSemBinaryDispatch::TrivialEq => VdMirApplicationFunction::TrivialEq,
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
                ref dispatch,
            } => match dispatch {
                VdSemLetterDispatch::Global(global_resolution) => match *global_resolution {
                    VdLetterGlobalResolution::Item(vd_item_path) => {
                        VdMirExprData::ItemPath(vd_item_path)
                    }
                },
                VdSemLetterDispatch::Local(local_defn) => VdMirExprData::Variable(VdMirVariable {}),
            },
            VdSemExprData::BaseOpr { opr } => todo!(),
            VdSemExprData::SeparatedList {
                items,
                ref dispatch,
                ..
            } => VdMirExprData::Application {
                function: match dispatch {
                    VdSemSeparatedListDispatch::Normal {
                        base_separator,
                        signature,
                    } => VdMirApplicationFunction::NormalSeparator,
                    VdSemSeparatedListDispatch::InSet { expr_ty } => {
                        VdMirApplicationFunction::InSet
                    }
                },
                arguments: items.to_vd_hir(self),
            },
            VdSemExprData::LxDelimited { item, .. } | VdSemExprData::Delimited { item, .. } => {
                self.build_expr(item)
            }
            VdSemExprData::Fraction {
                numerator,
                denominator,
                ..
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
