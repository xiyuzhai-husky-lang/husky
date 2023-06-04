use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConnectionKeyword {
    For,
    Where,
    Extends,
}

impl ConnectionKeyword {
    pub fn code(self) -> &'static str {
        match self {
            ConnectionKeyword::For => "for",
            ConnectionKeyword::Where => "where",
            ConnectionKeyword::Extends => "extends",
        }
    }
}

impl From<ConnectionKeyword> for Token {
    fn from(kw: ConnectionKeyword) -> Self {
        Token::Keyword(kw.into())
    }
}
