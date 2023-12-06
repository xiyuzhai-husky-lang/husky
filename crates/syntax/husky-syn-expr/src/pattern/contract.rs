use super::*;

impl SynPatternExpr {
    pub(super) fn contract(&self) -> Contract {
        match self {
            SynPatternExpr::Literal { .. } => Contract::Pure,
            SynPatternExpr::Ident {
                symbol_modifier_tokens,
                ..
            } => Contract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternExpr::TypeVariantUnit { .. } => Contract::Pure,
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            // ad hoc
            SynPatternExpr::OneOf { options } => Contract::Pure,
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
