use super::*;
use husky_syn_expr::pattern::{SynPatternData, SynPatternIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct PatternDeclarativeTypeInfo {
    ty: DecTerm,
}

impl PatternDeclarativeTypeInfo {
    fn new(ty: DecTerm) -> Self {
        Self { ty }
    }
}

impl<'a> DecTermEngine<'a> {
    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    pub(super) fn infer_pattern_expr_tys(&mut self, pattern_expr: SynPatternIdx, ty: DecTerm) {
        self.save_pattern_expr_ty(pattern_expr, ty);
        self.infer_subpattern_expr_tys(pattern_expr)
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_expr_ty(&mut self, pattern_idx: SynPatternIdx, ty: DecTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_idx, PatternDeclarativeTypeInfo::new(ty))
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_expr_tys(&mut self, pattern_idx: SynPatternIdx) {
        match self.syn_expr_region_data[pattern_idx] {
            SynPatternData::Literal { .. }
            | SynPatternData::Ident { .. }
            | SynPatternData::UnitTypeVariant { .. } => (), //  no subpatterns to infer
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { .. } => todo!(),
            SynPatternData::Props { name, ref fields } => todo!(),
            SynPatternData::OneOf { ref options } => todo!(),
            SynPatternData::Binding { .. } => todo!(),
            SynPatternData::Range { .. } => todo!(),
        }
    }

    pub(super) fn get_pattern_expr_ty(&self, pattern_idx: SynPatternIdx) -> Option<DecTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_idx)
            .map(|info| info.ty)
    }
}
