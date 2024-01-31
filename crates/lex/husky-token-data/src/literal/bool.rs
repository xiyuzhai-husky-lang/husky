use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BoolLiteralTokenData {
    True,
    False,
}

impl BoolLiteralTokenData {
    pub fn code(self) -> &'static str {
        match self {
            BoolLiteralTokenData::True => "true",
            BoolLiteralTokenData::False => "false",
        }
    }
}

impl From<BoolLiteralTokenData> for LiteralTokenData {
    fn from(value: BoolLiteralTokenData) -> Self {
        LiteralTokenData::Bool(value)
    }
}

impl From<BoolLiteralTokenData> for TokenData {
    fn from(value: BoolLiteralTokenData) -> Self {
        TokenData::Literal(value.into())
    }
}
