use super::*;

impl PatternExpr {
    pub(super) fn contract(&self) -> Contract {
        match self {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::Ident {
                symbol_modifier_keyword_group,
                ..
            } => Contract::new(*symbol_modifier_keyword_group),
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
        }
    }
}
