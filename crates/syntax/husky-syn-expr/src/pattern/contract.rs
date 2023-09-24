use super::*;

impl SynPatternExpr {
    pub(super) fn contract(&self) -> Contract {
        match self {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_tokens: symbol_modifier_keyword_group,
                ..
            } => Contract::new(*symbol_modifier_keyword_group),
            SynPatternExpr::TypeVariant { .. } => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { options } => todo!(),
            SynPatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }
}
