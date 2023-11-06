mod pattern_expr_ty;
mod pattern_symbol_ty;

pub(crate) use self::pattern_expr_ty::*;
pub(crate) use self::pattern_symbol_ty::*;

use super::*;

impl<'a> DeclarativeTermEngine<'a> {
    /// only use this for explicit parameters
    pub(super) fn infer_pattern_tys_in_parenate_parameter(
        &mut self,
        syn_pattern_expr_root: impl Into<SynPatternExprRoot> + Copy,
        ty: DeclarativeTerm,
    ) {
        let syn_pattern_expr_idx = syn_pattern_expr_root.into().syn_pattern_expr_idx();
        self.infer_pattern_expr_tys(syn_pattern_expr_idx, ty);
        self.infer_pattern_symbol_tys(syn_pattern_expr_root)
    }
}
