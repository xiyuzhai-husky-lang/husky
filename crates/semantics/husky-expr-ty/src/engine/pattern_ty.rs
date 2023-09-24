use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_pattern_and_symbols_ty(
        &mut self,
        pattern_expr_idx: SynPatternExprIdx,
        ty: FluffyTerm,
        symbols: CurrentSynSymbolIdxRange,
    ) {
        self.save_pattern_ty(pattern_expr_idx, ty);
        for symbol in symbols {
            self.infer_new_current_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_ty(&mut self, pattern_expr_idx: SynPatternExprIdx, ty: FluffyTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(pattern_expr_idx)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: SynPatternExprIdx) {
        match self.expr_region_data[pattern_expr_idx] {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident { .. } => (), // there is no subpattern to infer
            SynPatternExpr::Entity(_) => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { options } => todo!(),
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

    fn infer_new_current_symbol_ty(&mut self, current_symbol_idx: CurrentSynSymbolIdx) {
        if let Some(ty) = self.calc_new_current_symbol_ty(current_symbol_idx) {
            let ty = SymbolType::new(self, current_symbol_idx, ty);
            self.symbol_tys.insert_new(current_symbol_idx, ty)
        }
    }

    fn calc_new_current_symbol_ty(
        &mut self,
        current_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<FluffyTerm> {
        match self.expr_region_data[current_symbol_idx].variant() {
            CurrentSynSymbolVariant::TemplateParameter {
                template_parameter_variant,
                ..
            } => todo!(),
            CurrentSynSymbolVariant::ParenateRegularParameter {
                pattern_symbol_idx, ..
            } => todo!(),
            CurrentSynSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolVariant::BeVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolVariant::CaseVariable {
                pattern_symbol_idx, ..
            } => self.infer_new_pattern_symbol_ty(*pattern_symbol_idx),
            CurrentSynSymbolVariant::FrameVariable { .. } => todo!(),
            CurrentSynSymbolVariant::ParenateVariadicParameter { ident_token, .. } => todo!(),
            CurrentSynSymbolVariant::SelfType => todo!(),
            CurrentSynSymbolVariant::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentSynSymbolVariant::FieldVariable { ident_token } => todo!(),
        }
    }

    fn infer_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: SynPatternSymbolIdx,
    ) -> Option<FluffyTerm> {
        let ty_result = self.calc_new_pattern_symbol_ty(pattern_symbol_idx);
        let ty = ty_result.as_ref().ok().copied();
        self.pattern_symbol_ty_infos
            .insert_new(pattern_symbol_idx, PatternSymbolTypeInfo::new(ty_result));
        ty
    }

    fn calc_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: SynPatternSymbolIdx,
    ) -> PatternSymbolTypeResult<FluffyTerm> {
        match self.expr_region_data[pattern_symbol_idx] {
            SynPatternSymbol::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .ok_or(DerivedPatternSymbolTypeError::PatternExprTypeError.into()),
        }
    }

    fn get_pattern_expr_ty(&self, pattern_expr_idx: SynPatternExprIdx) -> Option<FluffyTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty().ok().copied())
            .flatten()
    }
}
