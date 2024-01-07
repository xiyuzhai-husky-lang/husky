use super::*;

impl SynPatternExprData {
    pub(super) fn contract(&self) -> TermContract {
        match self {
            SynPatternExprData::Literal { .. } => TermContract::Pure,
            SynPatternExprData::Ident {
                symbol_modifier_tokens,
                ..
            } => TermContract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternExprData::UnitTypeVariant { .. } => TermContract::Pure,
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            // ad hoc
            SynPatternExprData::TupleTypeVariant { .. } => TermContract::Pure,
            SynPatternExprData::Props { name: _, fields: _ } => todo!(),
            // ad hoc
            SynPatternExprData::OneOf { options: _ } => TermContract::Pure,
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
