use super::*;

impl<'a> RawTermEngine<'a> {
    pub(super) fn infer_pattern_contracts(&mut self) {
        for (idx, pattern) in self.expr_region_data.pattern_expr_arena().indexed_iter() {
            let contract = match pattern {
                PatternExpr::Literal(_) => todo!(),
                PatternExpr::Ident { modifier, .. } => (*modifier).into(),
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
            self.pattern_contracts.insert_new(idx, contract);
        }
    }
}
