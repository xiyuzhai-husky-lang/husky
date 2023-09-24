use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub struct PatternExprDeclarativeTypeInfo {
    ty: DeclarativeTerm,
}

impl PatternExprDeclarativeTypeInfo {
    fn new(ty: DeclarativeTerm) -> Self {
        Self { ty }
    }
}

impl<'a> DeclarativeTermEngine<'a> {
    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    pub(super) fn infer_pattern_expr_tys(
        &mut self,
        pattern_expr: SynPatternExprIdx,
        ty: DeclarativeTerm,
    ) {
        self.save_pattern_expr_ty(pattern_expr, ty);
        self.infer_subpattern_expr_tys(pattern_expr)
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_expr_ty(&mut self, pattern_expr_idx: SynPatternExprIdx, ty: DeclarativeTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprDeclarativeTypeInfo::new(ty))
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_expr_tys(&mut self, pattern_expr_idx: SynPatternExprIdx) {
        match self.syn_expr_region_data[pattern_expr_idx] {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident { .. } => (), // there is no subpattern to infer
            SynPatternExpr::TypeVariant { .. } => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { ref options } => todo!(),
            SynPatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    pub(super) fn get_pattern_expr_ty(
        &self,
        pattern_expr_idx: SynPatternExprIdx,
    ) -> Option<DeclarativeTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty)
    }
}
