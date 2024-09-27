mod pattern_ty;
mod pattern_variable_ty;

pub(crate) use self::pattern_ty::*;
pub(crate) use self::pattern_variable_ty::*;

use super::*;
use husky_syn_expr::context::SynPatternRoot;

impl<'a> DecTermEngine<'a> {
    /// only use this for explicit parameters
    pub(super) fn infer_pattern_tys_in_parenate_parameter(
        &mut self,
        syn_pattern_root: impl Into<SynPatternRoot> + Copy,
        ty: DecTerm,
    ) {
        let syn_pattern_idx = syn_pattern_root.into().syn_pattern_idx();
        self.infer_pattern_tys(syn_pattern_idx, ty);
        self.infer_pattern_variable_tys(syn_pattern_root)
    }
}
