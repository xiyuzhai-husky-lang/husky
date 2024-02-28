use crate::{HirEagerPatternExprIdx, ToHirEager};
use husky_hir_ty::HirType;
use husky_sema_expr::obelisks::closure_parameter::ClosureParameterObelisk;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerClosureParameterPattern {
    Simple {
        pattern_expr_idx: HirEagerPatternExprIdx,
        ty: Option<HirType>,
    },
}

impl ToHirEager for ClosureParameterObelisk {
    type Output = HirEagerClosureParameterPattern;

    fn to_hir_eager(&self, builder: &mut crate::HirEagerExprBuilder) -> Self::Output {
        match *self {
            ClosureParameterObelisk::Simple {
                syn_pattern_root,
                variables,
                ty,
                ..
            } => HirEagerClosureParameterPattern::Simple {
                pattern_expr_idx: builder.new_pattern_expr(syn_pattern_root),
                ty: ty.map(|ty| HirType::from_eth(builder.expr_term(ty), builder.db()).unwrap()),
            },
        }
    }
}
