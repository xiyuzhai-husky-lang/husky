use super::*;
use crate::pattern::VdSemPattern;
use visored_syn_expr::clause::r#let::placeholder::{
    VdSynLetClausePlaceholderType, VdSynLetPlaceholderResolution,
};
use visored_zfc_ty::{term::VdZfcTerm, ty::VdZfcType};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSemLetPlaceholderDispatch {
    pattern: VdSemPattern,
    ty: VdSemLetClausePlaceholderType,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSemLetClausePlaceholderType {
    Expr(VdSemExprIdx),
}

impl ToVdSem<VdSemLetPlaceholderDispatch> for &VdSynLetPlaceholderResolution {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemLetPlaceholderDispatch {
        let pattern = self.pattern().to_vd_sem(builder);
        let ty = self.ty().to_vd_sem(builder);
        let ty_term = builder.infer_pattern_ty_term(ty);
        builder.infer_pattern_symbol_tys(&pattern, ty_term.to_ty(builder.db()));
        VdSemLetPlaceholderDispatch { pattern, ty }
    }
}

impl ToVdSem<VdSemLetClausePlaceholderType> for VdSynLetClausePlaceholderType {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemLetClausePlaceholderType {
        match self {
            VdSynLetClausePlaceholderType::Expr(expr) => {
                VdSemLetClausePlaceholderType::Expr(expr.to_vd_sem(builder))
            }
        }
    }
}

impl<'db> VdSemExprBuilder<'db> {
    fn infer_pattern_ty_term(&mut self, ty: VdSemLetClausePlaceholderType) -> VdZfcTerm {
        match ty {
            VdSemLetClausePlaceholderType::Expr(expr) => self.infer_expr_term(expr),
        }
    }

    fn infer_pattern_symbol_tys(&mut self, pattern: &VdSemPattern, ty: VdZfcType) {
        match *pattern {
            VdSemPattern::Letter {
                token_idx_range,
                letter,
                local_defn,
            } => self.set_local_defn_ty(local_defn, ty),
        }
    }
}
