use husky_token::PatternSymbolModifierKeywordGroup;

use super::*;

impl<'a> RawTermEngine<'a> {
    pub(super) fn infer_pattern_symbol_modifiers(&mut self) {
        for (idx, symbol) in self
            .expr_region_data
            .pattern_expr_region()
            .pattern_symbol_arena()
            .indexed_iter()
        {
            let modifier = match symbol {
                PatternSymbol::Atom(expr_idx) => {
                    match self.expr_region_data.pattern_expr_arena()[expr_idx] {
                        PatternExpr::Ident {
                            modifier_keyword_group,
                            ident_token,
                        } => match modifier_keyword_group {
                            Some(modifier_keyword_group) => match modifier_keyword_group {
                                PatternSymbolModifierKeywordGroup::Mut(_) => todo!(),
                                PatternSymbolModifierKeywordGroup::RefMut(_, _) => todo!(),
                            },
                            None => SymbolModifier::Pure,
                        },
                        _ => unreachable!(),
                    }
                }
            };
            self.raw_term_symbol_region
                .add_new_pattern_symbol_modifier(idx, modifier)
        }
    }
}
