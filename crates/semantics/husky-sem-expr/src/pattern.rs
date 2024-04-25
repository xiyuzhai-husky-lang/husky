mod refutability;

use crate::*;
use husky_syn_expr::SynPatternData;

impl<'a> SemaExprBuilder<'a> {
    /// subpatterns get its type from its parent
    pub(crate) fn infer_subpattern_tys(&mut self, pattern_expr_idx: SynPatternIdx, ty: FlyTerm) {
        match self.syn_expr_region_data()[pattern_expr_idx] {
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

    pub(crate) fn calc_new_current_variable_ty(
        &mut self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<FlyTerm> {
        match self.syn_expr_region_data()[current_variable_idx].data() {
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

    pub(crate) fn calc_new_pattern_symbol_ty(
        &mut self,
        pattern_variable_idx: PatternVariableIdx,
    ) -> PatternSymbolTypeResult<FlyTerm> {
        match self.syn_expr_region_data()[pattern_variable_idx] {
            PatternVariable::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .ok_or(DerivedPatternSymbolTypeError::PatternSemaExprError.into()),
        }
    }
}
