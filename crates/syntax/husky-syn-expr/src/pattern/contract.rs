use super::*;

impl SynPatternExpr {
    pub(super) fn contract(&self) -> TermContract {
        match self {
            SynPatternExpr::Literal { .. } => TermContract::Pure,
            SynPatternExpr::Ident {
                symbol_modifier_tokens,
                ..
            } => TermContract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternExpr::TypeVariantUnit { .. } => TermContract::Pure,
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            // ad hoc
            SynPatternExpr::OneOf { options } => TermContract::Pure,
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
