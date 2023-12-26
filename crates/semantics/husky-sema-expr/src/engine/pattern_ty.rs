use super::*;

impl<'a> SemaExprEngine<'a> {
    /// used for defn body variables
    pub(crate) fn infer_variable_pattern_root_and_symbols_ty(
        &mut self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
        ty: FluffyTerm,
        symbols: CurrentSynSymbolIdxRange,
    ) {
        self.infer_pattern_ty(syn_pattern_root.into().syn_pattern_expr_idx(), ty);
        for symbol in symbols {
            self.infer_new_current_variable_syn_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn infer_pattern_ty(&mut self, syn_pattern_expr_idx: SynPatternExprIdx, ty: FluffyTerm) {
        self.pattern_expr_ty_infos
            .insert_new(syn_pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(syn_pattern_expr_idx, ty)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: SynPatternExprIdx, ty: FluffyTerm) {
        match self.syn_expr_region_data[pattern_expr_idx] {
            SynPatternExprData::Literal { .. } => (), // there is no subpattern to infer
            SynPatternExprData::Ident { .. } => (),   // there is no subpattern to infer
            SynPatternExprData::UnitTypeVariant { .. } => (), // there is no subpattern to infer
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { .. } =>
            /* ad hoc */
            {
                ()
            }
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { .. } => todo!(),
            SynPatternExprData::Props { name, ref fields } => todo!(),
            SynPatternExprData::OneOf { ref options } => {
                for option in options.elements() {
                    self.infer_pattern_ty(option.syn_pattern_expr_idx(), ty)
                }
            }
            SynPatternExprData::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExprData::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn infer_new_current_variable_syn_symbol_ty(
        &mut self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) {
        let Some(ty) = self.calc_new_current_syn_symbol_ty(current_syn_symbol_idx) else {
            return;
        };
        let modifier =
            match *self.syn_expr_region_data.symbol_region()[current_syn_symbol_idx].data() {
                CurrentSynSymbolData::LetVariable {
                    pattern_symbol_idx, ..
                }
                | CurrentSynSymbolData::BeVariable {
                    pattern_symbol_idx, ..
                }
                | CurrentSynSymbolData::CaseVariable {
                    pattern_symbol_idx, ..
                } => self
                    .expr_region_data()
                    .pattern_symbol_modifier(pattern_symbol_idx),
                _ => unreachable!(),
            };
        let ty = match SymbolType::new_variable_ty(self, current_syn_symbol_idx, modifier, ty) {
            Ok(ty) => ty,
            Err(_) => todo!(),
        };
        self.symbol_tys.insert_new(current_syn_symbol_idx, ty)
    }

    fn calc_new_current_syn_symbol_ty(
        &mut self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<FluffyTerm> {
        match self.syn_expr_region_data[current_syn_symbol_idx].data() {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant,
                ..
            } => todo!(),
            CurrentSynSymbolData::ParenateRegularParameter {
                pattern_symbol_idx, ..
            } => todo!(),
            CurrentSynSymbolData::LetVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::BeVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::CaseVariable {
                pattern_symbol_idx, ..
            } => self.infer_new_pattern_symbol_ty(*pattern_symbol_idx),
            CurrentSynSymbolData::LoopVariable { .. } => todo!(),
            CurrentSynSymbolData::ParenateVariadicParameter { ident_token, .. } => todo!(),
            CurrentSynSymbolData::SelfType => todo!(),
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentSynSymbolData::FieldVariable { ident_token } => todo!(),
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
        match self.syn_expr_region_data[pattern_symbol_idx] {
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
