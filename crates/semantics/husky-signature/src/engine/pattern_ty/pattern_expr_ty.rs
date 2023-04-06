use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PatternExprRawTypeInfo {
    ty: RawTerm,
}

impl PatternExprRawTypeInfo {
    fn new(ty: RawTerm) -> Self {
        Self { ty }
    }
}

impl<'a> RawTermEngine<'a> {
    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    pub(super) fn infer_pattern_expr_tys(&mut self, pattern_expr: PatternExprIdx, ty: RawTerm) {
        self.save_pattern_expr_ty(pattern_expr, ty);
        self.infer_subpattern_expr_tys(pattern_expr)
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_expr_ty(&mut self, pattern_expr_idx: PatternExprIdx, ty: RawTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprRawTypeInfo::new(ty))
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_expr_tys(&mut self, pattern_expr_idx: PatternExprIdx) {
        match self.expr_region_data[pattern_expr_idx] {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::Ident { .. } => (), // there is no subpattern to infer
            PatternExpr::Entity(_) => todo!(),
            PatternExpr::Tuple { name, fields } => todo!(),
            PatternExpr::Struct { name, fields } => todo!(),
            PatternExpr::OneOf { options } => todo!(),
            PatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    pub(super) fn get_pattern_expr_ty(&self, pattern_expr_idx: PatternExprIdx) -> Option<RawTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty)
    }
}
