use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_pattern_and_symbols_ty(
        &mut self,
        syn_pattern_root: SynPatternRoot,
        ty: FluffyTerm,
        symbols: SynCurrentSymbolIdxRange,
    ) {
        self.save_pattern_ty(syn_pattern_root.syn_pattern_expr_idx(), ty);
        for symbol in symbols {
            self.infer_new_current_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_ty(&mut self, syn_pattern_expr_idx: SynPatternExprIdx, ty: FluffyTerm) {
        self.pattern_expr_ty_infos
            .insert_new(syn_pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(syn_pattern_expr_idx)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: SynPatternExprIdx) {
        match self.expr_region_data[pattern_expr_idx] {
            SynPatternExpr::Literal { .. } => todo!(),
            SynPatternExpr::Ident { .. } => (), // there is no subpattern to infer
            SynPatternExpr::TypeVariantUnit { .. } => todo!(),
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

    fn infer_new_current_symbol_ty(&mut self, current_symbol_idx: SynCurrentSymbolIdx) {
        if let Some(ty) = self.calc_new_current_symbol_ty(current_symbol_idx) {
            let ty = SymbolType::new(self, current_symbol_idx, ty);
            self.symbol_tys.insert_new(current_symbol_idx, ty)
        }
    }

    fn calc_new_current_symbol_ty(
        &mut self,
        current_symbol_idx: SynCurrentSymbolIdx,
    ) -> Option<FluffyTerm> {
        match self.expr_region_data[current_symbol_idx].variant() {
            SynCurrentSymbolVariant::TemplateParameter {
                template_parameter_variant,
                ..
            } => todo!(),
            SynCurrentSymbolVariant::ParenateRegularParameter {
                pattern_symbol_idx, ..
            } => todo!(),
            SynCurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            }
            | SynCurrentSymbolVariant::BeVariable {
                pattern_symbol_idx, ..
            }
            | SynCurrentSymbolVariant::CaseVariable {
                pattern_symbol_idx, ..
            } => self.infer_new_pattern_symbol_ty(*pattern_symbol_idx),
            SynCurrentSymbolVariant::FrameVariable { .. } => todo!(),
            SynCurrentSymbolVariant::ParenateVariadicParameter { ident_token, .. } => todo!(),
            SynCurrentSymbolVariant::SelfType => todo!(),
            SynCurrentSymbolVariant::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            SynCurrentSymbolVariant::FieldVariable { ident_token } => todo!(),
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
                .ok_or(DerivedPatternSymbolTypeError::PatternSemaExprError.into()),
        }
    }

    fn get_pattern_expr_ty(&self, pattern_expr_idx: SynPatternExprIdx) -> Option<FluffyTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty().ok().copied())
            .flatten()
    }
}
