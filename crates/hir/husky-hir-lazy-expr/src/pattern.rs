use husky_hir_ty::HirType;
use husky_sem_expr::LetVariableObelisk;
use husky_syn_expr::{BePatternSyndicate, SynPatternData};

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
        let_variables_pattern: &LetVariableObelisk,
    ) -> HirLazyLetVariablesPattern {
        HirLazyLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.syn_pattern_root()),
            variables: let_variables_pattern
                .variables()
                .into_iter()
                .filter_map(|var| self.current_syn_symbol_to_hir_lazy_variable(var))
                .collect(),
            ty: let_variables_pattern
                .ty_sem_expr_idx()
                .map(|ty_sem_expr_idx| {
                    HirType::from_eth(self.expr_term(ty_sem_expr_idx), self.db()).unwrap()
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

impl ToHirLazy for BePatternSyndicate {
    type Output = HirLazyBeVariablesPattern;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let db = builder.db();
        let pattern_expr_arena = builder.syn_expr_region_data().pattern_expr_arena();
        match pattern_expr_arena[self.syn_pattern_root().syn_pattern_expr_idx()] {
            SynPatternData::Literal {
                regional_token_idx: _,
                literal: _,
            } => todo!(),
            SynPatternData::Ident {
                symbol_modifier_tokens: _,
                ident_token: _,
            } => todo!(),
            SynPatternData::UnitTypeVariant {
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
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { path, .. } => {
                // ad hoc
                if path.ident(db).data(db) == "Some" {
                    HirLazyBeVariablesPattern::Some
                } else {
                    todo!()
                }
            }
            SynPatternData::Props { name: _, fields: _ } => todo!(),
            SynPatternData::OneOf { options: _ } => todo!(),
            SynPatternData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
