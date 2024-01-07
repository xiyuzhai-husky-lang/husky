use husky_hir_ty::HirType;
use husky_sema_expr::LetPatternSemaSyndicate;
use husky_syn_expr::{BePatternSynSyndicate, SynPatternExprData};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyLetVariablesPattern {
    pattern_expr_idx: HirLazyPatternExprIdx,
    variables: SmallVec<[HirLazyVariableIdx; 2]>,
    // variables: CurrentHirLazySymbolIdxRange,
    ty: Option<HirType>,
}

impl HirLazyLetVariablesPattern {
    pub fn pattern_expr_idx(&self) -> HirLazyPatternExprIdx {
        self.pattern_expr_idx
    }

    pub fn variables(&self) -> &[HirLazyVariableIdx] {
        &self.variables
    }
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_variables_pattern: &LetPatternSemaSyndicate,
    ) -> HirLazyLetVariablesPattern {
        HirLazyLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.syn_pattern_root()),
            variables: let_variables_pattern
                .variables()
                .into_iter()
                .filter_map(|var| self.current_syn_symbol_to_hir_lazy_variable(var))
                .collect(),
            ty: let_variables_pattern
                .ty_sema_expr_idx()
                .map(|ty_sema_expr_idx| {
                    HirType::from_ethereal(self.expr_term(ty_sema_expr_idx), self.db()).unwrap()
                }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirLazyBeVariablesPattern {
    Literal,
    None,
    Some,
}

impl ToHirLazy for BePatternSynSyndicate {
    type Output = HirLazyBeVariablesPattern;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let db = builder.db();
        let pattern_expr_arena = builder.syn_expr_region_data().pattern_expr_arena();
        match pattern_expr_arena[self.syn_pattern_root().syn_pattern_expr_idx()] {
            SynPatternExprData::Literal {
                regional_token_idx: _,
                literal: _,
            } => todo!(),
            SynPatternExprData::Ident {
                symbol_modifier_tokens: _,
                ident_token: _,
            } => todo!(),
            SynPatternExprData::UnitTypeVariant {
                path_expr_idx: _,
                path,
            } => {
                // ad hoc
                if path.ident(db).data(db) == "None" {
                    HirLazyBeVariablesPattern::None
                } else {
                    todo!()
                }
            }
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { path, .. } => {
                // ad hoc
                if path.ident(db).data(db) == "Some" {
                    HirLazyBeVariablesPattern::Some
                } else {
                    todo!()
                }
            }
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { .. } => todo!(),
            SynPatternExprData::Props { name: _, fields: _ } => todo!(),
            SynPatternExprData::OneOf { options: _ } => todo!(),
            SynPatternExprData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExprData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
