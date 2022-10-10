use crate::*;
use husky_opn_syntax::RawOpnVariant;
use husky_primitive_literal_syntax::RawLiteralData;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawVariableKind {
    Normal { init_range: TextRange },
    ThisField,
    Unrecognized,
}
