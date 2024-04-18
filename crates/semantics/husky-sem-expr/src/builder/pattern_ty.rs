use husky_coword::Ident;

use super::*;

impl<'a> SemaExprBuilder<'a> {
    /// used for defn body variables
    pub(crate) fn infer_variable_pattern_root_and_symbols_ty(
        &mut self,
        syn_pattern_root: impl Into<SynPatternRoot>,
        ty: FlyTerm,
        symbols: CurrentSynSymbolIdxRange,
    ) {
        self.infer_pattern_ty(syn_pattern_root.into().syn_pattern_expr_idx(), ty);
        for symbol in symbols {
            self.infer_new_current_variable_syn_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn infer_pattern_ty(&mut self, syn_pattern_expr_idx: SynPatternIdx, ty: FlyTerm) {
        self.pattern_expr_ty_infos
            .insert_new(syn_pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(syn_pattern_expr_idx, ty)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: SynPatternIdx, ty: FlyTerm) {
        match self.syn_expr_region_data[pattern_expr_idx] {
            SynPatternData::Literal { .. } => (), // there is no subpattern to infer
            SynPatternData::Ident { .. } => (),   // there is no subpattern to infer
            SynPatternData::UnitTypeVariant { .. } => (), // there is no subpattern to infer
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { .. } =>
            /* ad hoc */
            {
                ()
            }
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { .. } => todo!(),
            SynPatternData::Props { name, ref fields } => todo!(),
            SynPatternData::OneOf { ref options } => {
                for option in options.elements() {
                    self.infer_pattern_ty(option.syn_pattern(), ty)
                }
            }
            SynPatternData::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternData::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn infer_new_current_variable_syn_symbol_ty(
        &mut self,
        current_syn_symbol_idx: CurrentVariableIdx,
    ) {
        let Some(ty) = self.calc_new_current_syn_symbol_ty(current_syn_symbol_idx) else {
            return;
        };
        let modifier =
            match *self.syn_expr_region_data.symbol_region()[current_syn_symbol_idx].data() {
                CurrentVariableData::SimpleClosureParameter {
                    pattern_variable_idx,
                    ..
                }
                | CurrentVariableData::LetVariable {
                    pattern_variable_idx,
                    ..
                }
                | CurrentVariableData::BeVariable {
                    pattern_variable_idx,
                    ..
                }
                | CurrentVariableData::CaseVariable {
                    pattern_variable_idx,
                    ..
                } => self
                    .expr_region_data()
                    .pattern_symbol_modifier(pattern_variable_idx),
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
        current_syn_symbol_idx: CurrentVariableIdx,
    ) -> Option<FlyTerm> {
        match self.syn_expr_region_data[current_syn_symbol_idx].data() {
            CurrentVariableData::TemplateParameter {
                data: template_parameter_variant,
                ..
            } => todo!(),
            CurrentVariableData::SimpleParenateParameter {
                pattern_variable_idx,
                ..
            } => todo!(),
            CurrentVariableData::VariadicParenateParameter { ident_token, .. } => todo!(),
            CurrentVariableData::SimpleClosureParameter {
                ident,
                pattern_variable_idx,
            } => self.infer_new_pattern_symbol_ty(*pattern_variable_idx),
            CurrentVariableData::LetVariable {
                pattern_variable_idx,
                ..
            }
            | CurrentVariableData::BeVariable {
                pattern_variable_idx,
                ..
            }
            | CurrentVariableData::CaseVariable {
                pattern_variable_idx,
                ..
            } => self.infer_new_pattern_symbol_ty(*pattern_variable_idx),
            CurrentVariableData::LoopVariable { .. } => todo!(),
            CurrentVariableData::SelfType => todo!(),
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentVariableData::FieldVariable { ident_token } => todo!(),
        }
    }

    fn infer_new_pattern_symbol_ty(
        &mut self,
        pattern_variable_idx: PatternVariableIdx,
    ) -> Option<FlyTerm> {
        let ty_result = self.calc_new_pattern_symbol_ty(pattern_variable_idx);
        let ty = ty_result.as_ref().ok().copied();
        self.pattern_symbol_ty_infos
            .insert_new(pattern_variable_idx, PatternSymbolTypeInfo::new(ty_result));
        ty
    }

    fn calc_new_pattern_symbol_ty(
        &mut self,
        pattern_variable_idx: PatternVariableIdx,
    ) -> PatternSymbolTypeResult<FlyTerm> {
        match self.syn_expr_region_data[pattern_variable_idx] {
            PatternVariable::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .ok_or(DerivedPatternSymbolTypeError::PatternSemaExprError.into()),
        }
    }

    fn get_pattern_expr_ty(&self, pattern_expr_idx: SynPatternIdx) -> Option<FlyTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty().ok().copied())
            .flatten()
    }
}
