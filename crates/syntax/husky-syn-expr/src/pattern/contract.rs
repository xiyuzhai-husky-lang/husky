use super::*;

impl PatternSynExpr {
    pub(super) fn contract(&self) -> Contract {
        match self {
            PatternSynExpr::Literal(_) => todo!(),
            PatternSynExpr::Ident {
                symbol_modifier_keyword_group,
                ..
            } => Contract::new(*symbol_modifier_keyword_group),
            PatternSynExpr::Entity(_) => todo!(),
            PatternSynExpr::Tuple { name, fields } => todo!(),
            PatternSynExpr::Struct { name, fields } => todo!(),
            PatternSynExpr::OneOf { options } => todo!(),
            PatternSynExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternSynExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }
}
