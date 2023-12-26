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
            SynPatternExprData::Props { name, ref fields } => todo!(),
            // ad hoc
            SynPatternExprData::OneOf { options } => TermContract::Pure,
            SynPatternExprData::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExprData::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }
}
