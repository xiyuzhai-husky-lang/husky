use super::*;

impl SynPatternExprData {
    pub(super) fn contract(&self) -> Contract {
        match self {
            SynPatternExprData::Literal { .. } => Contract::Pure,
            SynPatternExprData::Ident {
                symbol_modifier_tokens,
                ..
            } => Contract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternExprData::UnitTypeVariant { .. } => Contract::Pure,
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            // ad hoc
            SynPatternExprData::TupleTypeVariant { .. } => Contract::Pure,
            SynPatternExprData::Props { name: _, fields: _ } => todo!(),
            // ad hoc
            SynPatternExprData::OneOf { options: _ } => Contract::Pure,
            SynPatternExprData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExprData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
