use super::*;

impl SynPatternData {
    pub(super) fn contract(&self) -> Contract {
        match self {
            SynPatternData::Literal { .. } => Contract::Pure,
            SynPatternData::Ident {
                symbol_modifier_tokens,
                ..
            } => Contract::new(*symbol_modifier_tokens),
            // ad hoc
            SynPatternData::UnitTypeVariant { .. } => Contract::Pure,
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            // ad hoc
            SynPatternData::TupleTypeVariant { .. } => Contract::Pure,
            SynPatternData::Props { name: _, fields: _ } => todo!(),
            // ad hoc
            SynPatternData::OneOf { options: _ } => Contract::Pure,
            SynPatternData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
