use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct PatternExprDeclarativeTypeInfo {
    ty: DecTerm,
}

impl PatternExprDeclarativeTypeInfo {
    fn new(ty: DecTerm) -> Self {
        Self { ty }
    }
}

impl<'a> DecTermEngine<'a> {
    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    pub(super) fn infer_pattern_expr_tys(&mut self, pattern_expr: PatternSynExprIdx, ty: DecTerm) {
        self.save_pattern_expr_ty(pattern_expr, ty);
        self.infer_subpattern_expr_tys(pattern_expr)
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_expr_ty(&mut self, pattern_expr_idx: PatternSynExprIdx, ty: DecTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprDeclarativeTypeInfo::new(ty))
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_expr_tys(&mut self, pattern_expr_idx: PatternSynExprIdx) {
        match self.syn_expr_region_data[pattern_expr_idx] {
            SynPatternExprData::Literal { .. }
            | SynPatternExprData::Ident { .. }
            | SynPatternExprData::UnitTypeVariant { .. } => (), //  no subpatterns to infer
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { .. } => todo!(),
            SynPatternExprData::Props { name, ref fields } => todo!(),
            SynPatternExprData::OneOf { ref options } => todo!(),
            SynPatternExprData::Binding { .. } => todo!(),
            SynPatternExprData::Range { .. } => todo!(),
        }
    }

    pub(super) fn get_pattern_expr_ty(
        &self,
        pattern_expr_idx: PatternSynExprIdx,
    ) -> Option<DecTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty)
    }
}
