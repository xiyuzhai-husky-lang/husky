use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BoolLiteralData {
    True,
    False,
}

impl BoolLiteralData {
    pub fn code(self) -> &'static str {
        match self {
            BoolLiteralData::True => "true",
            BoolLiteralData::False => "false",
        }
    }
}

impl From<BoolLiteralData> for LiteralData {
    fn from(value: BoolLiteralData) -> Self {
        LiteralData::Bool(value)
    }
}

impl From<BoolLiteralData> for TokenData {
    fn from(value: BoolLiteralData) -> Self {
        TokenData::Literal(value.into())
    }
}
