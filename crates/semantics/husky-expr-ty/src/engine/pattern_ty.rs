use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_pattern_and_symbols_ty(
        &mut self,
        pattern_expr_idx: PatternSynExprIdx,
        ty: FluffyTerm,
        symbols: CurrentSymbolIdxRange,
    ) {
        self.save_pattern_ty(pattern_expr_idx, ty);
        for symbol in symbols {
            self.infer_new_current_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_ty(&mut self, pattern_expr_idx: PatternSynExprIdx, ty: FluffyTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(pattern_expr_idx)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: PatternSynExprIdx) {
        match self.expr_region_data[pattern_expr_idx] {
            PatternSynExpr::Literal(_) => todo!(),
            PatternSynExpr::Ident { .. } => (), // there is no subpattern to infer
            PatternSynExpr::Entity(_) => todo!(),
            PatternSynExpr::Tuple { name, fields } => todo!(),
            PatternSynExpr::Struct { name, fields } => todo!(),
            PatternSynExpr::OneOf { options } => todo!(),
            PatternSynExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternSynExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn infer_new_current_symbol_ty(&mut self, current_symbol_idx: CurrentSymbolIdx) {
        if let Some(ty) = self.calc_new_current_symbol_ty(current_symbol_idx) {
            let ty = SymbolType::new(self, current_symbol_idx, ty);
            self.symbol_tys.insert_new(current_symbol_idx, ty)
        }
    }

    fn calc_new_current_symbol_ty(
        &mut self,
        current_symbol_idx: idx_arena::ArenaIdx<CurrentSymbol>,
    ) -> Option<FluffyTerm> {
        match self.expr_region_data[current_symbol_idx].variant() {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => todo!(),
            CurrentSymbolVariant::ExplicitRegularParameter {
                pattern_symbol_idx, ..
            } => todo!(),
            CurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => self.infer_new_pattern_symbol_ty(*pattern_symbol_idx),
            CurrentSymbolVariant::FrameVariable { .. } => todo!(),
            CurrentSymbolVariant::ExplicitVariadicParameter { ident_token, .. } => todo!(),
        }
    }

    fn infer_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: PatternSymbolIdx,
    ) -> Option<FluffyTerm> {
        let ty_result = self.calc_new_pattern_symbol_ty(pattern_symbol_idx);
        let ty = ty_result.as_ref().ok().copied();
        self.pattern_symbol_ty_infos
            .insert_new(pattern_symbol_idx, PatternSymbolTypeInfo::new(ty_result));
        ty
    }

    fn calc_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: PatternSymbolIdx,
    ) -> PatternSymbolTypeResult<FluffyTerm> {
        match self.expr_region_data[pattern_symbol_idx] {
            PatternSynSymbol::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .ok_or(DerivedPatternSymbolTypeError::PatternExprTypeError.into()),
        }
    }

    fn get_pattern_expr_ty(&self, pattern_expr_idx: PatternSynExprIdx) -> Option<FluffyTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty().ok().copied())
            .flatten()
    }
}
