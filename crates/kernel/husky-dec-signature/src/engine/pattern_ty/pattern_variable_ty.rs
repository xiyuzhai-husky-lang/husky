use super::*;
use husky_syn_expr::{
    context::SynPatternRoot,
    pattern::{PatternVariable, PatternVariableIdx},
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DecPatternVariableTypeInfo {
    modifier: VariableModifier,
    base_ty: DecTerm,
}

impl DecPatternVariableTypeInfo {
    fn new(modifier: VariableModifier, base_ty: DecTerm) -> Self {
        Self { modifier, base_ty }
    }

    pub fn modifier(&self) -> VariableModifier {
        self.modifier
    }

    pub fn base_ty(&self) -> DecTerm {
        self.base_ty
    }
}

impl<'a> DecTermEngine<'a> {
    pub(super) fn infer_pattern_variable_tys(
        &mut self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) {
        let syn_pattern_root = syn_pattern_root.into();
        for (_, pattern_variable) in self
            .syn_expr_region_data
            .pattern_region()
            .pattern_variables(syn_pattern_root.syn_pattern_idx())
        {
            self.infer_new_pattern_variable_ty(*pattern_variable)
        }
    }

    fn infer_new_pattern_variable_ty(&mut self, pattern_variable_idx: PatternVariableIdx) {
        let modifier = self
            .syn_expr_region_data
            .pattern_variable_modifier(pattern_variable_idx);
        let base_ty = self.calc_new_pattern_variable_base_ty(pattern_variable_idx);
        self.pattern_variable_ty_infos.insert_new(
            pattern_variable_idx,
            DecPatternVariableTypeInfo::new(modifier, base_ty),
        )
    }

    fn calc_new_pattern_variable_base_ty(
        &mut self,
        pattern_variable: PatternVariableIdx,
    ) -> DecTerm {
        match self.syn_expr_region_data[pattern_variable] {
            PatternVariable::Atom(pattern) => self
                .get_pattern_ty(pattern)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
