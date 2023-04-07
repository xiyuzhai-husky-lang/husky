use super::*;

impl<'a> RawTermEngine<'a> {
    pub(super) fn infer_symbol_modifiers(&mut self) {
        self.infer_pattern_symbol_modifiers();
        self.infer_current_symbol_modifiers()
    }

    pub(super) fn infer_current_symbol_modifiers(&mut self) {
        for (current_symbol_idx, current_symbol) in self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter()
        {
            let modifier = match current_symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => SymbolModifier::StaticExpr,
                CurrentSymbolVariant::ExplicitParameter {
                    ident,
                    pattern_symbol,
                } => todo!(),
                CurrentSymbolVariant::LetVariable {
                    ident,
                    pattern_symbol_idx,
                } => todo!(),
                CurrentSymbolVariant::FrameVariable { ident, expr_idx } => todo!(),
            };
            self.raw_term_symbol_region
                .add_new_current_symbol_modifier(current_symbol_idx, modifier)
        }
    }

    fn infer_pattern_symbol_modifiers(&mut self) {
        for (idx, pattern) in self.expr_region_data.pattern_expr_arena().indexed_iter() {
            let modifier = match pattern {
                PatternExpr::Literal(_) => todo!(),
                PatternExpr::Ident { modifier, .. } => *modifier,
                PatternExpr::Entity(_) => todo!(),
                PatternExpr::Tuple { name, fields } => todo!(),
                PatternExpr::Struct { name, fields } => todo!(),
                PatternExpr::OneOf { options } => todo!(),
                PatternExpr::Binding {
                    ident_token,
                    asperand_token,
                    src,
                } => todo!(),
                PatternExpr::Range {
                    start,
                    dot_dot_token,
                    end,
                } => todo!(),
            };
            todo!()
            // self.pattern_contracts.insert_new(idx, modifier);
        }
    }
}
