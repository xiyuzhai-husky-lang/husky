use crate::*;
use husky_opn_syntax::RawOpnVariant;
use husky_primitive_literal_syntax::RawLiteralData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Literal(RawLiteralData),
    Variable {
        varname: Identifier,
        variable_kind: RawVariableKind,
    },
    Bracketed(RawExprIdx),
    Opn {
        opn_variant: RawOpnVariant,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RawExprIdx>)>,
        RawExprIdx,
    ),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawVariableKind {
    Normal { init_range: TextRange },
    ThisField,
    Unrecognized,
}
