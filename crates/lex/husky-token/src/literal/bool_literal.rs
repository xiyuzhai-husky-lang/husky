use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BoolLiteral {
    True,
    False,
}

impl BoolLiteral {
    pub fn code(self) -> &'static str {
        match self {
            BoolLiteral::True => "true",
            BoolLiteral::False => "false",
        }
    }
}

impl From<BoolLiteral> for Literal {
    fn from(value: BoolLiteral) -> Self {
        Literal::Bool(value)
    }
}

impl From<BoolLiteral> for Token {
    fn from(value: BoolLiteral) -> Self {
        Token::Literal(value.into())
    }
}
