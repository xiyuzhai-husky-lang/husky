use super::*;

impl SynPatternExpr {
    pub(super) fn contract(&self) -> TermContract {
        match self {
            SynPatternExpr::Literal { .. } => TermContract::None,
            SynPatternExpr::Ident {
                symbol_modifier_tokens,
                ..
            } => TermContract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternExpr::TypeVariantUnit { .. } => TermContract::None,
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            // ad hoc
            SynPatternExpr::OneOf { options } => TermContract::None,
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
